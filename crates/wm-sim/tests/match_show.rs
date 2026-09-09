use proptest::prelude::*;
use wm_domain::game::*;
use wm_sim::{
    content::{base_pack, generate_world, initial_agents},
    planning::{agent_plan, validate_plan},
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
    Session::new(seed, 1, card, workers, agents, 65, vec![])
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
