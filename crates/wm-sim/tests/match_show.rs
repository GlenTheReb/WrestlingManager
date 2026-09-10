use proptest::prelude::*;
use wm_domain::game::*;
use wm_domain::match_rules::*;
use wm_sim::{
    content::{base_pack, generate_world, initial_agents},
    planning::{agent_plan, validate_engine_support, validate_plan},
    runtime::Session,
};

fn session(seed: u64) -> Session {
    let workers = generate_world(seed, &base_pack());
    let agents = initial_agents();
    let plan = MatchPlan {
        match_type: "Singles".into(),
        worker_a: workers[0].id.clone(),
        worker_b: workers[1].id.clone(),
        winner_id: Some(workers[0].id.clone()),
        finish: Finish::Pinfall,
        clean_finish: true,
        duration_seconds: 600,
        style: "Technical".into(),
        pace: 3,
        risk: 3,
        freedom: 65,
        purpose: "Competitive match".into(),
        protected_worker_id: None,
        agent_id: agents[0].id,
        beats: vec![],
    };
    let plan = agent_plan(plan, &workers[0], &workers[1], &agents[0])
        .unwrap()
        .plan;
    let card = vec![
        Segment {
            id: 1,
            title: "Opener".into(),
            content: SegmentPlan::Match(plan.clone()),
        },
        Segment {
            id: 2,
            title: "Challenge".into(),
            content: SegmentPlan::Angle(AnglePlan {
                participants: vec![workers[2].id.clone()],
                purpose: "Challenge".into(),
                duration_seconds: 90,
            }),
        },
        Segment {
            id: 3,
            title: "Main event".into(),
            content: SegmentPlan::Match(plan),
        },
    ];
    Session::new(seed, 1, card, workers, agents, 65, vec![]).unwrap()
}

// FNV-1a is only a compact regression fingerprint, not a security checksum.
fn fingerprint(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

#[test]
fn generated_duplicate_names_use_clear_deterministic_ordinals() {
    let workers = generate_world(42, &base_pack());
    let names = workers
        .iter()
        .map(|worker| worker.name.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(names.len(), workers.len());
    assert!(names.iter().any(|name| name.ends_with(" II")));
    assert!(names.iter().all(|name| {
        !name
            .split_whitespace()
            .next_back()
            .is_some_and(|suffix| suffix.chars().all(|character| character.is_ascii_digit()))
    }));
}

#[test]
fn engine_0_4_0_singles_golden_output() {
    let mut sim = session(42);
    let mut events = sim.advance(347);
    let snapshot = serde_json::to_vec(&sim).unwrap();
    let partial = fingerprint(&snapshot);
    let mut sim: Session = serde_json::from_slice(&snapshot).unwrap();
    events.extend(sim.advance(3600));
    let final_state = fingerprint(&serde_json::to_vec(&sim).unwrap());
    let event_stream = fingerprint(&serde_json::to_vec(&events).unwrap());
    // WM-022 changes the serialized worker identity; match event output stays unchanged.
    assert_eq!(event_stream, 0xa741f1eb4e898d39, "event stream drift");
    assert_eq!(events.len(), 108);
    assert_eq!(
        (partial, final_state),
        (11415513778334552897, 4471617209222952147),
        "identity snapshot drift"
    );
}

#[test]
fn identities_are_deterministic_valid_and_do_not_stereotype_personality_by_background() {
    let a = generate_world(42, &base_pack());
    let b = generate_world(42, &base_pack());
    assert_eq!(a, b);
    assert_ne!(a[0].identity, generate_world(43, &base_pack())[0].identity);
    for worker in &a {
        worker.identity.validate().unwrap();
        assert_eq!(worker.identity.languages[0].name, worker.language);
        assert!(worker.identity.hobbies.len() <= 5);
    }
    let mut alternate = base_pack();
    alternate.backgrounds[0].nationality = "Changed nationality".into();
    let c = generate_world(42, &alternate);
    assert_eq!(a[0].identity.personality, c[0].identity.personality);
}

#[test]
fn every_legacy_finish_converts_and_simulates_without_changing_the_booked_result() {
    for match_type in ["Singles", "No disqualification"] {
        for finish in [
            Finish::Pinfall,
            Finish::Submission,
            Finish::CountOut,
            Finish::Disqualification,
            Finish::Draw,
            Finish::NoContest,
        ] {
            for winner_b in [false, true] {
                let source = session(9);
                let mut card = vec![source.card[0].clone()];
                let SegmentPlan::Match(plan) = &mut card[0].content else {
                    unreachable!()
                };
                plan.match_type = match_type.into();
                plan.finish = finish.clone();
                plan.winner_id = finish.decision_method().map(|_| {
                    if winner_b {
                        plan.worker_b.clone()
                    } else {
                        plan.worker_a.clone()
                    }
                });
                let saved_plan = serde_json::to_value(&plan).unwrap();
                let definition = MatchDefinition::try_from(&*plan).unwrap();
                validate_engine_support(&definition).unwrap();
                let winners: Vec<_> = definition.winning_worker_ids().map(str::to_owned).collect();
                assert_eq!(
                    winners,
                    plan.winner_id.clone().into_iter().collect::<Vec<_>>()
                );
                assert_eq!(serde_json::to_value(&plan).unwrap(), saved_plan);
                let expected_winner = plan.winner_id.clone();
                let mut sim =
                    Session::new(9, 1, card, source.workers, source.agents, 65, vec![]).unwrap();
                sim.advance(3600);
                assert!(sim.complete);
                assert_eq!(sim.reports[0].winner_id, expected_winner);
                assert_eq!(sim.reports[0].changes.len(), 2);
            }
        }
    }
}

#[test]
fn future_formats_are_structurally_valid_but_not_silently_simulated_as_singles() {
    let source = session(1);
    let SegmentPlan::Match(plan) = &source.card[0].content else {
        unreachable!()
    };
    let singles = MatchDefinition::try_from(plan).unwrap();
    for size in [2, 3] {
        for participation in [ParticipationRule::Tag, ParticipationRule::AllActive] {
            let mut team = singles.clone();
            for side in ["a", "b"] {
                for member in 1..size {
                    team.slots.push(ParticipantSlot {
                        id: format!("slot-{side}-{member}"),
                        side_id: format!("side-{side}"),
                        worker_id: format!("additional-{side}-{member}"),
                    });
                }
            }
            team.rules.participation = participation;
            team.validate().unwrap();
            assert!(
                validate_engine_support(&team)
                    .unwrap_err()
                    .to_string()
                    .contains("not available yet")
            );
        }
    }
    let mut elimination = singles.clone();
    elimination.rules.victory = VictoryRule::Elimination;
    elimination.validate().unwrap();
    assert!(validate_engine_support(&elimination).is_err());
    let mut three_way = singles;
    three_way.sides.push(MatchSide {
        id: "side-c".into(),
    });
    three_way.slots.push(ParticipantSlot {
        id: "slot-c".into(),
        side_id: "side-c".into(),
        worker_id: "third-worker".into(),
    });
    three_way.validate().unwrap();
    assert!(validate_engine_support(&three_way).is_err());

    for label in ["Tag", "Trios", "Triple threat", "Singles typo"] {
        let mut card = source.card.clone();
        let SegmentPlan::Match(plan) = &mut card[0].content else {
            unreachable!()
        };
        plan.match_type = label.into();
        assert!(
            Session::new(
                1,
                1,
                card,
                source.workers.clone(),
                source.agents.clone(),
                65,
                vec![]
            )
            .is_err()
        );
    }
}

#[test]
fn simulator_rejects_invalid_booked_results_and_missing_people_before_start() {
    let source = session(2);
    for (finish, winner) in [
        (Finish::Pinfall, None),
        (Finish::Submission, Some("outsider".to_owned())),
        (Finish::Draw, Some(source.workers[0].id.clone())),
        (Finish::NoContest, Some(source.workers[1].id.clone())),
    ] {
        let mut card = source.card.clone();
        let SegmentPlan::Match(plan) = &mut card[0].content else {
            unreachable!()
        };
        plan.finish = finish;
        plan.winner_id = winner;
        assert!(
            Session::new(
                2,
                1,
                card,
                source.workers.clone(),
                source.agents.clone(),
                65,
                vec![]
            )
            .is_err()
        );
    }
    assert!(
        Session::new(
            2,
            1,
            source.card.clone(),
            vec![],
            source.agents.clone(),
            65,
            vec![]
        )
        .is_err()
    );
    assert!(Session::new(2, 1, source.card, source.workers, vec![], 65, vec![]).is_err());
}

#[test]
fn single_ticks_fast_playback_and_serialised_resume_are_identical() {
    let mut instant = session(42);
    let all_events = instant.advance(3600);
    let mut slow = session(42);
    let mut slow_events = vec![];
    for _ in 0..347 {
        slow_events.extend(slow.advance(1));
    }
    let json = serde_json::to_string(&slow).unwrap();
    let mut resumed: Session = serde_json::from_str(&json).unwrap();
    while !resumed.complete {
        slow_events.extend(resumed.advance(8));
    }
    assert_eq!(instant, resumed);
    assert_eq!(all_events, slow_events);
    assert!(
        all_events
            .windows(2)
            .all(|p| p[0].sequence + 1 == p[1].sequence)
    );
    assert_eq!(instant.reports.len(), 3);
    assert!(instant.advance(100).is_empty());
    // Every narrated move must exist in the actual actor's repertoire.
    for event in all_events.iter().filter(|e| e.move_id.is_some()) {
        let actor = instant
            .workers
            .iter()
            .find(|w| Some(&w.id) == event.actor_id.as_ref())
            .unwrap();
        assert!(
            actor
                .moves
                .iter()
                .any(|m| Some(&m.id) == event.move_id.as_ref())
        );
    }
}

#[test]
fn live_instructions_arrive_later_and_cannot_change_the_booked_winner() {
    let mut sim = session(9);
    sim.advance(30);
    let pace = sim.current.as_ref().unwrap().plan.pace;
    sim.queue(LiveInstruction {
        kind: InstructionKind::SlowDown,
        worker_id: None,
        finish: None,
        beat_index: None,
    })
    .unwrap();
    assert_eq!(sim.current.as_ref().unwrap().plan.pace, pace);
    let events = sim.advance(30);
    assert!(events.iter().any(|e| e.kind == "instruction"));
    assert_eq!(sim.current.as_ref().unwrap().plan.pace, pace - 1);
    assert!(
        sim.queue(LiveInstruction {
            kind: InstructionKind::ChangeFinish,
            worker_id: None,
            finish: Some(Finish::Draw),
            beat_index: None
        })
        .is_err()
    );
    let winner = sim.current.as_ref().unwrap().plan.winner_id.clone();
    sim.queue(LiveInstruction {
        kind: InstructionKind::ChangeFinish,
        worker_id: None,
        finish: Some(Finish::Submission),
        beat_index: None,
    })
    .unwrap();
    sim.advance(3600);
    assert_eq!(sim.reports[0].winner_id, winner);
    assert!(sim.reports[0].result.contains("submission"));
}

#[test]
fn earlier_show_state_changes_the_next_match_and_survives_the_break() {
    let mut normal = session(21);
    normal.advance(690);
    let mut disappointed = normal.clone();
    disappointed.crowd.peak = 100;
    for cohort in &mut disappointed.crowd.cohorts {
        cohort.trust = 10;
    }
    normal.advance(600);
    disappointed.advance(600);
    assert!(
        disappointed.reports[2].performance.engagement < normal.reports[2].performance.engagement
    );
    assert_eq!(normal.crowd.fatigue, 21);
    assert_eq!(normal.crowd.cohorts.len(), 3);
}

#[test]
fn repeated_interference_damages_trust_and_youth_learns_more() {
    let mut sim = session(3);
    let SegmentPlan::Match(plan) = &mut sim.card[0].content else {
        unreachable!()
    };
    plan.beats = (1..=6)
        .map(|n| PlannedBeat {
            at_second: n * 60,
            actor_id: plan.worker_a.clone(),
            kind: BeatKind::Interference,
            move_id: None,
            duration_seconds: 0,
        })
        .collect();
    sim.advance(600);
    assert_eq!(sim.crowd.interference_count, 6);
    assert!(sim.crowd.trust < 65);
    let changes = &sim.reports[0].changes;
    assert!(changes[0].development > changes[1].development);
    assert!(changes[1].wear > changes[0].wear);
}

#[test]
fn advice_preserves_manual_booking_and_rejects_foreign_moves() {
    let sim = session(1);
    let SegmentPlan::Match(mut plan) = sim.card[0].content.clone() else {
        unreachable!()
    };
    let original = plan.clone();
    let advice = agent_plan(
        plan.clone(),
        &sim.workers[0],
        &sim.workers[1],
        &sim.agents[0],
    )
    .unwrap();
    assert_eq!(advice.plan.winner_id, original.winner_id);
    for beat in original.beats {
        assert!(advice.plan.beats.contains(&beat));
    }
    plan.duration_seconds = 2100;
    assert!(validate_plan(&plan, &sim.workers[0], &sim.workers[1], &sim.agents[0]).is_ok());
    plan.beats[0].move_id = Some("not-in-repertoire".into());
    assert!(validate_plan(&plan, &sim.workers[0], &sim.workers[1], &sim.agents[0]).is_err());
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn booked_results_and_bounded_conditions_hold_across_worlds(seed in any::<u64>()) {
        let mut sim = session(seed);
        let winner = sim.workers[0].id.clone();
        sim.advance(3600);
        for report in [&sim.reports[0], &sim.reports[2]] { prop_assert_eq!(report.winner_id.as_ref(), Some(&winner)); }
        for worker in sim.workers { prop_assert!((0..=100).contains(&worker.condition.fatigue)); }
        prop_assert!((0..=100).contains(&sim.crowd.energy));
    }
}
