use rusqlite::{Connection, params};
use std::collections::BTreeSet;
use tempfile::tempdir;
use wm_domain::{
    CreateGameRequest,
    relationships::{
        InteractionKind, InteractionRequest, PersonalRelationshipEvent, RelationshipDelta,
        RelationshipMemory, RelationshipMemoryKind,
    },
};
use wm_persistence::SaveRepository;

fn create(directory: &std::path::Path) -> SaveRepository {
    let repo = SaveRepository::new(directory);
    repo.create_game(CreateGameRequest {
        save_id: "relationships".into(),
        seed: "42".into(),
    })
    .unwrap();
    repo
}

fn request(
    id: &str,
    worker: &str,
    kind: InteractionKind,
    revision: u32,
    context: Option<String>,
) -> InteractionRequest {
    InteractionRequest {
        save_id: "relationships".into(),
        request_id: id.into(),
        worker_id: worker.into(),
        kind,
        context_worker_id: context,
        expected_revision: revision,
    }
}

#[test]
fn generated_relationships_are_directional_visible_and_reloadable() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let roster = repo.roster_page("relationships", "", 0, 64).unwrap();
    let a = &roster.rows[0];
    let profile = repo.worker_profile("relationships", &a.id).unwrap();
    assert_eq!(profile.relationships.rule_version, 1);
    assert_eq!(profile.relationships.personal.len(), 12);
    assert_eq!(profile.relationships.management.revision, 0);
    assert_eq!(profile.relationships.attention_remaining, 4);
    assert_eq!(
        SaveRepository::new(directory.path())
            .worker_profile("relationships", &a.id)
            .unwrap()
            .relationships,
        profile.relationships
    );
}

#[test]
fn new_saves_are_sparse_and_colleague_targets_are_bounded_searchable_and_pageable() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let roster = repo.roster_page("relationships", "", 0, 64).unwrap();
    let subject = &roster.rows[0];
    let path = directory.path().join("relationships.sqlite3");
    let db = Connection::open(path).unwrap();
    let relationship_rows: u32 = db
        .query_row("SELECT COUNT(*) FROM personal_relationships", [], |row| {
            row.get(0)
        })
        .unwrap();
    let memory_rows: u32 = db
        .query_row("SELECT COUNT(*) FROM relationship_memories", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(relationship_rows <= roster.total * 2);
    assert_eq!(relationship_rows, memory_rows);
    drop(db);

    let oversized = repo
        .interaction_targets("relationships", &subject.id, "", 0, 1_000)
        .unwrap();
    assert_eq!(oversized.total, roster.total - 1);
    assert_eq!(oversized.rows.len(), 32);

    let mut reached = BTreeSet::new();
    let mut offset = 0;
    while offset < oversized.total {
        let page = repo
            .interaction_targets("relationships", &subject.id, "", offset, 7)
            .unwrap();
        assert!(page.rows.len() <= 7);
        reached.extend(page.rows.into_iter().map(|target| target.worker_id));
        offset += 7;
    }
    let expected = roster
        .rows
        .iter()
        .filter(|worker| worker.id != subject.id)
        .map(|worker| worker.id.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(reached, expected);

    let searched_worker = roster.rows.last().unwrap();
    let searched = repo
        .interaction_targets("relationships", &subject.id, &searched_worker.name, 0, 7)
        .unwrap();
    assert!(
        searched
            .rows
            .iter()
            .any(|target| target.worker_id == searched_worker.id)
    );
}

#[test]
fn conversations_are_contextual_idempotent_and_protected_from_repeats() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let roster = repo.roster_page("relationships", "", 0, 64).unwrap();
    let worker = &roster.rows[0];
    let colleague = &roster.rows[1];
    let introduced = repo
        .interact_with_worker(request(
            "introduce-1",
            &worker.id,
            InteractionKind::IntroduceYourself,
            0,
            None,
        ))
        .unwrap();
    assert!(
        introduced
            .response
            .contains(worker.name.split(' ').next().unwrap())
    );
    assert_eq!(
        repo.interact_with_worker(request(
            "introduce-1",
            &worker.id,
            InteractionKind::IntroduceYourself,
            0,
            None,
        ))
        .unwrap(),
        introduced
    );
    assert!(
        repo.interact_with_worker(request(
            "introduce-1",
            &worker.id,
            InteractionKind::CheckIn,
            1,
            None,
        ))
        .is_err()
    );
    assert!(
        repo.interact_with_worker(request(
            "stale",
            &worker.id,
            InteractionKind::CheckIn,
            0,
            None,
        ))
        .is_err()
    );
    let checked = repo
        .interact_with_worker(request(
            "check-in-1",
            &worker.id,
            InteractionKind::CheckIn,
            1,
            None,
        ))
        .unwrap();
    assert_ne!(checked.response, introduced.response);
    let discussed = repo
        .interact_with_worker(request(
            "colleague-1",
            &worker.id,
            InteractionKind::DiscussColleague,
            2,
            Some(colleague.id.clone()),
        ))
        .unwrap();
    assert!(discussed.response.contains(&colleague.name));
    let profile = repo.worker_profile("relationships", &worker.id).unwrap();
    assert_eq!(profile.relationships.interaction_history.len(), 3);
    assert_eq!(profile.relationships.attention_remaining, 1);
    assert!(
        !profile
            .relationships
            .interaction_options
            .iter()
            .find(|option| option.kind == InteractionKind::IntroduceYourself)
            .unwrap()
            .enabled
    );
    let check_in = profile
        .relationships
        .interaction_options
        .iter()
        .find(|option| option.kind == InteractionKind::CheckIn)
        .unwrap();
    assert!(!check_in.enabled);
    assert!(
        check_in
            .unavailable_reason
            .as_deref()
            .unwrap()
            .contains("repetitive")
    );
    let news = repo
        .news_page("relationships", "People", false, 0, 20)
        .unwrap();
    assert_eq!(news.total, 3);
    assert!(
        news.items
            .iter()
            .all(|item| item.worker_id == Some(worker.id.clone()))
    );
}

#[test]
fn encouragement_applies_and_audits_clamped_morale_and_confidence_changes() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let worker = repo.roster_page("relationships", "", 0, 1).unwrap().rows[0].clone();
    let path = directory.path().join("relationships.sqlite3");
    let db = Connection::open(&path).unwrap();
    db.execute(
        "UPDATE workers SET condition=json_set(condition,'$.morale',99,'$.confidence',10) WHERE id=?1",
        [&worker.id],
    )
    .unwrap();
    db.execute(
        "UPDATE management_relationships SET state=json_set(state,'$.scores.affinity',99,'$.scores.trust',99) WHERE worker_id=?1",
        [&worker.id],
    )
    .unwrap();
    drop(db);

    let outcome = repo
        .interact_with_worker(request(
            "encourage-confidence",
            &worker.id,
            InteractionKind::OfferEncouragement,
            0,
            None,
        ))
        .unwrap();
    assert_eq!(outcome.morale_delta, 1);
    assert!(outcome.confidence_delta > 0);
    assert_eq!(outcome.relationship_delta.affinity, 1);
    assert_eq!(outcome.relationship_delta.trust, 1);
    outcome.validate().unwrap();

    let reloaded = repo.worker_profile("relationships", &worker.id).unwrap();
    assert_eq!(reloaded.worker.condition.morale, 100);
    assert_eq!(
        reloaded.worker.condition.confidence,
        10 + outcome.confidence_delta
    );
    assert_eq!(reloaded.relationships.interaction_history[0], outcome);

    let db = Connection::open(&path).unwrap();
    db.execute(
        "UPDATE player_interactions SET outcome=json_set(outcome,'$.confidenceDelta',999) WHERE request_id='encourage-confidence'",
        [],
    )
    .unwrap();
    drop(db);
    assert!(repo.worker_profile("relationships", &worker.id).is_err());
}

#[test]
fn attention_budget_and_context_requirements_are_enforced_server_side() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let roster = repo.roster_page("relationships", "", 0, 64).unwrap();
    let a = &roster.rows[0];
    let b = &roster.rows[1];
    assert!(
        repo.interact_with_worker(request(
            "missing-target",
            &a.id,
            InteractionKind::DiscussColleague,
            0,
            None,
        ))
        .is_err()
    );
    let actions = [
        ("a-intro", &a.id, InteractionKind::IntroduceYourself, 0),
        ("b-intro", &b.id, InteractionKind::IntroduceYourself, 0),
        ("a-check", &a.id, InteractionKind::CheckIn, 1),
        ("b-check", &b.id, InteractionKind::CheckIn, 1),
    ];
    for (id, worker, kind, revision) in actions {
        repo.interact_with_worker(request(id, worker, kind, revision, None))
            .unwrap();
    }
    let profile = repo.worker_profile("relationships", &a.id).unwrap();
    assert_eq!(profile.relationships.attention_remaining, 0);
    assert!(
        profile
            .relationships
            .interaction_options
            .iter()
            .all(|option| !option.enabled)
    );
    assert!(
        repo.interact_with_worker(request(
            "over-budget",
            &a.id,
            InteractionKind::AskForCreativeInput,
            2,
            None,
        ))
        .is_err()
    );
}

#[test]
fn personal_events_change_only_the_recorded_direction_and_keep_memory() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let roster = repo.roster_page("relationships", "", 0, 64).unwrap();
    let a = &roster.rows[0];
    let b = &roster.rows[1];
    let event = PersonalRelationshipEvent {
        memory: RelationshipMemory {
            id: "support-a-b".into(),
            subject_id: a.id.clone(),
            other_id: b.id.clone(),
            occurred_on: "2026-01-01".into(),
            kind: RelationshipMemoryKind::Support,
            summary: "They stood up for them during a difficult meeting.".into(),
            impact: RelationshipDelta {
                affinity: 70,
                respect: 20,
                trust: 35,
                tension: 0,
            },
            salience: 75,
            active_until: Some("2026-12-31".into()),
            source: "test-event".into(),
        },
    };
    let mut future = event.clone();
    future.memory.id = "future-support-a-b".into();
    future.memory.occurred_on = "2026-01-02".into();
    assert!(
        repo.record_personal_relationship_event("relationships", future, 0)
            .is_err()
    );
    let changed = repo
        .record_personal_relationship_event("relationships", event.clone(), 0)
        .unwrap();
    assert_eq!(changed.revision, 1);
    assert_eq!(
        repo.record_personal_relationship_event("relationships", event.clone(), 0)
            .unwrap(),
        changed
    );
    let mut conflict = event.clone();
    conflict.memory.summary = "Different event with the same ID.".into();
    assert!(
        repo.record_personal_relationship_event("relationships", conflict, 1)
            .is_err()
    );
    let profile = repo.worker_profile("relationships", &a.id).unwrap();
    let relation = profile
        .relationships
        .personal
        .iter()
        .find(|relationship| relationship.other_id == b.id)
        .unwrap();
    assert!(
        relation
            .memories
            .iter()
            .any(|memory| memory.id == "support-a-b")
    );
    let reverse = repo.worker_profile("relationships", &b.id).unwrap();
    assert!(reverse.relationships.personal.iter().all(|relationship| {
        relationship
            .memories
            .iter()
            .all(|memory| memory.id != "support-a-b")
    }));
}

#[test]
fn tension_decays_on_scheduled_days_without_erasing_relationship_history() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let roster = repo.roster_page("relationships", "", 0, 2).unwrap();
    let a = &roster.rows[0];
    let b = &roster.rows[1];
    let path = directory.path().join("relationships.sqlite3");
    let db = Connection::open(&path).unwrap();
    db.execute("UPDATE promotions SET current_date='2026-01-14'", [])
        .unwrap();
    db.execute("UPDATE shows SET show_date='2026-02-01'", [])
        .unwrap();
    drop(db);

    let changed = repo
        .record_personal_relationship_event(
            "relationships",
            PersonalRelationshipEvent {
                memory: RelationshipMemory {
                    id: "conflict-a-b".into(),
                    subject_id: a.id.clone(),
                    other_id: b.id.clone(),
                    occurred_on: "2026-01-14".into(),
                    kind: RelationshipMemoryKind::Disagreement,
                    summary: "A disagreement raised tension between them.".into(),
                    impact: RelationshipDelta {
                        affinity: 0,
                        respect: 0,
                        trust: 0,
                        tension: 40,
                    },
                    salience: 60,
                    active_until: Some("2026-07-14".into()),
                    source: "test-event".into(),
                },
            },
            0,
        )
        .unwrap();
    assert!(changed.scores.tension > 0);

    repo.continue_day("relationships").unwrap();
    let db = Connection::open(&path).unwrap();
    let (tension, revision): (i32, u32) = db
        .query_row(
            "SELECT json_extract(state,'$.scores.tension'), json_extract(state,'$.revision') FROM personal_relationships WHERE subject_id=?1 AND other_id=?2",
            params![a.id, b.id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(tension, changed.scores.tension - 1);
    assert_eq!(revision, changed.revision + 1);
    let memory_count: i32 = db
        .query_row(
            "SELECT COUNT(*) FROM relationship_memories WHERE id='conflict-a-b'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(memory_count, 1);
}

#[test]
fn schema_five_upgrade_is_backed_up_and_invalid_state_is_rejected() {
    let directory = tempdir().unwrap();
    let repo = create(directory.path());
    let path = directory.path().join("relationships.sqlite3");
    let db = Connection::open(&path).unwrap();
    db.execute_batch("DROP TABLE character_action_receipts; DROP TABLE character_changes; DROP TABLE audience_response_evidence; DROP TABLE character_aliases; DROP TABLE character_tenures; DROP TABLE characters; DROP TABLE persons; DROP TABLE worker_blacklist; DROP TABLE worker_shortlist_members; DROP TABLE worker_shortlists; DROP TABLE worker_saved_views; DROP TABLE worker_discovery_index; DROP TABLE player_interactions; DROP TABLE management_relationships; DROP TABLE relationship_memories; DROP TABLE personal_relationships; UPDATE metadata SET value='5' WHERE key='schema_version'; UPDATE metadata SET value='0.4.0' WHERE key='engine_version'; PRAGMA user_version=5;").unwrap();
    drop(db);
    let upgraded = repo.load_game("relationships").unwrap();
    assert_eq!(upgraded.schema_version, 8);
    assert_eq!(upgraded.engine_version, "0.7.0");
    assert!(
        directory
            .path()
            .join("relationships.sqlite3.v5.bak")
            .exists()
    );
    let worker = repo.roster_page("relationships", "", 0, 1).unwrap().rows[0]
        .id
        .clone();
    let db = Connection::open(&path).unwrap();
    db.execute(
        "UPDATE management_relationships SET state=json_set(state,'$.scores.tension',999) WHERE worker_id=?1",
        params![worker],
    )
    .unwrap();
    drop(db);
    assert!(repo.worker_profile("relationships", &worker).is_err());
}
