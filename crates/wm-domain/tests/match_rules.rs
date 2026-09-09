use proptest::prelude::*;
use wm_domain::match_rules::*;

fn booking(sizes: &[usize], participation: ParticipationRule) -> MatchDefinition {
    MatchDefinition {
        sides: (0..sizes.len())
            .map(|i| MatchSide {
                id: format!("side-{i}"),
            })
            .collect(),
        slots: sizes
            .iter()
            .enumerate()
            .flat_map(|(side, size)| {
                (0..*size).map(move |member| ParticipantSlot {
                    id: format!("slot-{side}-{member}"),
                    worker_id: format!("worker-{side}-{member}"),
                    side_id: format!("side-{side}"),
                })
            })
            .collect(),
        rules: MatchRules {
            participation,
            victory: VictoryRule::OneFall,
        },
        result: BookedResult::Decision {
            winning_side_id: "side-0".into(),
            method: DecisionMethod::Pinfall,
            deciding_slot_id: Some("slot-0-0".into()),
            defeated_slot_id: Some("slot-1-0".into()),
        },
    }
}

#[test]
fn singles_teams_handicap_tornado_and_multi_side_matches_share_one_contract() {
    for (sizes, participation) in [
        (vec![1, 1], ParticipationRule::AllActive),
        (vec![2, 2], ParticipationRule::Tag),
        (vec![3, 3], ParticipationRule::Tag),
        (vec![1, 2], ParticipationRule::Tag),
        (vec![2, 2], ParticipationRule::AllActive),
        (vec![1, 1, 1, 1], ParticipationRule::AllActive),
        (vec![2, 2, 2], ParticipationRule::Tag),
    ] {
        let mut definition = booking(&sizes, participation);
        for victory in [VictoryRule::OneFall, VictoryRule::Elimination] {
            definition.rules.victory = victory;
            definition.validate().unwrap();
            assert_eq!(definition.winning_worker_ids().count(), sizes[0]);
        }
    }
}

#[test]
fn ambiguous_identities_missing_sides_and_impossible_tags_are_rejected() {
    let valid = booking(&[2, 2], ParticipationRule::Tag);
    let mut bad = valid.clone();
    bad.slots[1].worker_id = bad.slots[0].worker_id.clone();
    assert_eq!(bad.validate(), Err(MatchRuleError::Identity));
    bad = valid.clone();
    bad.slots[1].id = bad.slots[0].id.clone();
    assert_eq!(bad.validate(), Err(MatchRuleError::Identity));
    bad = valid.clone();
    bad.sides[1].id = bad.sides[0].id.clone();
    assert_eq!(bad.validate(), Err(MatchRuleError::Identity));
    for id in ["", " ", " worker-0-0"] {
        bad = valid.clone();
        bad.slots[0].worker_id = id.into();
        assert_eq!(bad.validate(), Err(MatchRuleError::Identity));
    }
    bad = valid.clone();
    bad.slots[0].side_id = "unknown".into();
    assert_eq!(bad.validate(), Err(MatchRuleError::Membership));
    bad = valid.clone();
    bad.sides.push(MatchSide {
        id: "empty-side".into(),
    });
    assert_eq!(bad.validate(), Err(MatchRuleError::OpposingSides));
    bad = booking(&[2], ParticipationRule::Tag);
    assert_eq!(bad.validate(), Err(MatchRuleError::OpposingSides));
    bad = booking(&[1, 1], ParticipationRule::Tag);
    assert_eq!(bad.validate(), Err(MatchRuleError::TagPartners));
}

#[test]
fn decisive_result_requires_real_opponents_and_a_participating_winning_side() {
    for (winning, deciding, defeated, expected) in [
        (
            "unknown",
            Some("slot-0-0"),
            Some("slot-1-0"),
            MatchRuleError::WinningSide,
        ),
        (
            "side-0",
            Some("unknown"),
            Some("slot-1-0"),
            MatchRuleError::DecidingSlot,
        ),
        (
            "side-0",
            Some("slot-1-0"),
            Some("slot-1-1"),
            MatchRuleError::DecidingSlot,
        ),
        (
            "side-0",
            Some("slot-0-0"),
            Some("unknown"),
            MatchRuleError::DefeatedSlot,
        ),
        (
            "side-0",
            Some("slot-0-0"),
            Some("slot-0-1"),
            MatchRuleError::DefeatedSlot,
        ),
        (
            "side-0",
            None,
            Some("slot-1-0"),
            MatchRuleError::FallParticipants,
        ),
        (
            "side-0",
            Some("slot-0-0"),
            None,
            MatchRuleError::FallParticipants,
        ),
    ] {
        for method in [DecisionMethod::Pinfall, DecisionMethod::Submission] {
            let mut definition = booking(&[2, 2], ParticipationRule::Tag);
            definition.result = BookedResult::Decision {
                winning_side_id: winning.into(),
                method,
                deciding_slot_id: deciding.map(str::to_owned),
                defeated_slot_id: defeated.map(str::to_owned),
            };
            assert_eq!(definition.validate(), Err(expected.clone()));
        }
    }
}

#[test]
fn whole_side_decisions_and_no_winner_results_do_not_invent_a_fall() {
    let mut definition = booking(&[3, 3], ParticipationRule::Tag);
    for method in [DecisionMethod::CountOut, DecisionMethod::Disqualification] {
        definition.result = BookedResult::Decision {
            winning_side_id: "side-0".into(),
            method,
            deciding_slot_id: None,
            defeated_slot_id: None,
        };
        definition.validate().unwrap();
        assert_eq!(definition.winning_worker_ids().count(), 3);
    }
    for result in [BookedResult::Draw {}, BookedResult::NoContest {}] {
        definition.result = result;
        definition.validate().unwrap();
        assert_eq!(definition.winning_side_id(), None);
        assert_eq!(definition.winning_worker_ids().count(), 0);
    }
}

#[test]
fn result_json_matches_the_shared_contract_and_rejects_contradictions() {
    let definition = booking(&[2, 2], ParticipationRule::Tag);
    let json = serde_json::to_value(&definition).unwrap();
    assert_eq!(
        json["result"],
        serde_json::json!({
            "outcome": "decision", "winningSideId": "side-0", "method": "pinfall",
            "decidingSlotId": "slot-0-0", "defeatedSlotId": "slot-1-0"
        })
    );
    assert_eq!(
        serde_json::from_value::<MatchDefinition>(json).unwrap(),
        definition
    );
    for json in [
        r#"{"outcome":"draw","winningSideId":"side-0"}"#,
        r#"{"outcome":"noContest","defeatedSlotId":"slot-1-0"}"#,
        r#"{"outcome":"decision","winningSideId":"side-0","method":"draw"}"#,
    ] {
        assert!(
            serde_json::from_str::<BookedResult>(json).is_err(),
            "accepted contradictory result: {json}"
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn side_membership_and_results_survive_display_reordering(
        sizes in prop::collection::vec(1usize..5, 2..6),
    ) {
        let mut definition = booking(&sizes, ParticipationRule::AllActive);
        definition.validate().unwrap();
        let mut winners: Vec<_> = definition.winning_worker_ids().map(str::to_owned).collect();
        winners.sort();
        definition.slots.reverse();
        definition.sides.reverse();
        definition.validate().unwrap();
        let mut reordered: Vec<_> = definition.winning_worker_ids().map(str::to_owned).collect();
        reordered.sort();
        prop_assert_eq!(winners, reordered);
        // Substituting a wrestler does not change the match-local slot/result.
        let result = definition.result.clone();
        definition.slots[0].worker_id = "substitute".into();
        definition.validate().unwrap();
        prop_assert_eq!(result, definition.result.clone());
        let duplicate = definition.slots[0].worker_id.clone();
        definition.slots[1].worker_id = duplicate;
        prop_assert_eq!(definition.validate(), Err(MatchRuleError::Identity));
    }
}
