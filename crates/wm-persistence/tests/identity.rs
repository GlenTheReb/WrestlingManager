use rusqlite::{Connection, params};
use tempfile::tempdir;
use wm_domain::{CreateGameRequest, identity::*, traits::*};
use wm_persistence::SaveRepository;
fn create(dir: &std::path::Path) -> SaveRepository {
    let r = SaveRepository::new(dir);
    r.create_game(CreateGameRequest {
        save_id: "identity".into(),
        seed: "42".into(),
    })
    .unwrap();
    r
}
fn event(id: &str) -> IdentityEvent {
    IdentityEvent {
        id: id.into(),
        incident_id: format!("incident-{id}"),
        person_id: "worker-001".into(),
        kind: IdentityEventKind::MissedMediaBooking,
        occurred_on: "2026-01-01".into(),
        recorded_on: "2026-01-01".into(),
        company_id: None,
        visibility: IdentityVisibility::Public,
        outcome: EvidenceOutcome::Confirmed,
        source: "Verified fictional fixture".into(),
        supersedes: None,
        recognition: None,
        service_days: None,
    }
}

#[test]
fn identity_roundtrip_projection_and_language_search() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let first = repo.worker_profile("identity", "worker-001").unwrap();
    assert!(first.worker.identity.motivations.is_some());
    assert!(!first.biography.is_empty());
    assert!(!first.personality_description.partial);
    assert_eq!(
        SaveRepository::new(dir.path())
            .worker_profile("identity", "worker-001")
            .unwrap(),
        first
    );
    assert!(
        repo.roster_page("identity", &first.worker.identity.languages[0].name, 0, 64)
            .unwrap()
            .total
            > 0
    );
    let mut identity = first.worker.identity.clone();
    let integrity = identity
        .personality
        .get_mut(&PersonalityField::Integrity)
        .unwrap();
    integrity.visibility = IdentityVisibility::Hidden;
    integrity.source = "Secret case".into();
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    db.execute(
        "UPDATE workers SET identity=?1 WHERE id='worker-001'",
        [serde_json::to_string(&identity).unwrap()],
    )
    .unwrap();
    let profile = repo.worker_profile("identity", "worker-001").unwrap();
    assert!(
        profile.worker.identity.personality[&PersonalityField::Integrity]
            .value
            .is_none()
    );
    assert!(
        !serde_json::to_string(&profile)
            .unwrap()
            .contains("Secret case")
    );
    identity.hobbies = vec![
        Interest {
            hobby: Hobby::Music,
            involvement: Involvement::Casual
        };
        2
    ];
    db.execute(
        "UPDATE workers SET identity=?1 WHERE id='worker-001'",
        [serde_json::to_string(&identity).unwrap()],
    )
    .unwrap();
    assert!(repo.worker_profile("identity", "worker-001").is_err());
}

#[test]
fn roster_description_uses_company_authorised_assessments() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let mut identity = repo
        .worker_profile("identity", "worker-001")
        .unwrap()
        .worker
        .identity;
    for assessment in identity.personality.values_mut() {
        assessment.value = None;
        assessment.source.clear();
    }
    let ambition = identity
        .personality
        .get_mut(&PersonalityField::Ambition)
        .unwrap();
    ambition.value = Some(wm_domain::ratings::Rating100::new(90).unwrap());
    ambition.visibility = IdentityVisibility::Company("uwf".into());
    ambition.source = "Confidential company assessment".into();
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    db.execute(
        "UPDATE workers SET identity=?1 WHERE id='worker-001'",
        [serde_json::to_string(&identity).unwrap()],
    )
    .unwrap();

    let row = repo
        .roster_page("identity", "", 0, 64)
        .unwrap()
        .rows
        .into_iter()
        .find(|row| row.id == "worker-001")
        .unwrap();
    assert_eq!(row.personality_description, "Ambitious");
    assert_eq!(
        repo.worker_profile("identity", "worker-001")
            .unwrap()
            .personality_description
            .text,
        row.personality_description
    );
}

#[test]
fn profile_rejects_trait_state_that_does_not_match_its_evidence() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    repo.record_identity_event("identity", event("only-one"), 0)
        .unwrap();
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    let json: String = db
        .query_row(
            "SELECT ledger FROM identity_traits WHERE worker_id='worker-001'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let mut ledger: TraitLedger = serde_json::from_str(&json).unwrap();
    ledger.states[0].status = TraitStatus::Elevated;
    db.execute(
        "UPDATE identity_traits SET ledger=?1 WHERE worker_id='worker-001'",
        [serde_json::to_string(&ledger).unwrap()],
    )
    .unwrap();

    assert!(repo.worker_profile("identity", "worker-001").is_err());
}

#[test]
fn evidence_transactions_reject_stale_conflicting_and_missing_subject_writes() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let a = event("a");
    let ledger = repo
        .record_identity_event("identity", a.clone(), 0)
        .unwrap();
    assert_eq!(ledger.revision, 1);
    assert_eq!(
        repo.record_identity_event("identity", a.clone(), 0)
            .unwrap(),
        ledger
    );
    assert!(
        repo.record_identity_event("identity", event("b"), 0)
            .is_err()
    );
    let mut bad = a;
    bad.source = "different".into();
    assert!(repo.record_identity_event("identity", bad, 1).is_err());
    let mut absent = event("absent");
    absent.person_id = "missing".into();
    assert!(repo.record_identity_event("identity", absent, 0).is_err());
    repo.record_identity_event("identity", event("b"), 1)
        .unwrap();
    repo.record_identity_event("identity", event("c"), 2)
        .unwrap();
    let restarted = SaveRepository::new(dir.path());
    let view = restarted.worker_profile("identity", "worker-001").unwrap();
    assert_eq!(view.exceptional_traits.states.len(), 1);
    assert_eq!(view.exceptional_traits.history.len(), 1);
    let mut correction = event("a");
    correction.id = "a-correction".into();
    correction.supersedes = Some("a".into());
    correction.outcome = EvidenceOutcome::Retracted;
    repo.record_identity_event("identity", correction, 3)
        .unwrap();
    let view = repo.worker_profile("identity", "worker-001").unwrap();
    assert!(view.exceptional_traits.states.is_empty());
    assert_eq!(view.exceptional_traits.history.len(), 2);
}

#[test]
fn continue_day_expires_traits_and_preserves_identity() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        repo.record_identity_event("identity", event(id), i as u32)
            .unwrap();
    }
    let before = repo
        .worker_profile("identity", "worker-001")
        .unwrap()
        .worker
        .identity;
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    let d = chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap() + chrono::Days::new(539);
    db.execute("UPDATE promotions SET current_date=?1", [d.to_string()])
        .unwrap();
    db.execute("UPDATE shows SET show_date='2028-01-01'", [])
        .unwrap();
    repo.continue_day("identity").unwrap();
    let after = repo.worker_profile("identity", "worker-001").unwrap();
    assert_eq!(after.worker.identity, before);
    assert!(after.exceptional_traits.states.is_empty());
    assert_eq!(after.exceptional_traits.history.len(), 2);
}

fn downgrade_v4(db: &Connection) {
    db.execute_batch("DROP TABLE character_action_receipts; DROP TABLE character_changes; DROP TABLE audience_response_evidence; DROP TABLE character_aliases; DROP TABLE character_tenures; DROP TABLE characters; DROP TABLE persons; DROP TABLE worker_blacklist; DROP TABLE worker_shortlist_members; DROP TABLE worker_shortlists; DROP TABLE worker_saved_views; DROP TABLE worker_discovery_index; DROP TABLE player_interactions; DROP TABLE management_relationships; DROP TABLE relationship_memories; DROP TABLE personal_relationships; DROP TABLE identity_traits; ALTER TABLE workers DROP COLUMN identity; UPDATE metadata SET value='4' WHERE key='schema_version'; UPDATE metadata SET value='0.3.0' WHERE key='engine_version'; PRAGMA user_version=4;").unwrap();
}
#[test]
fn schema_four_upgrade_preserves_original_and_marks_unrecorded_facts_unknown() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let before = repo.worker_profile("identity", "worker-001").unwrap();
    let path = dir.path().join("identity.sqlite3");
    let db = Connection::open(&path).unwrap();
    downgrade_v4(&db);
    drop(db);
    let updated = repo.load_game("identity").unwrap();
    assert_eq!(updated.schema_version, 8);
    assert_eq!(updated.engine_version, "0.7.0");
    let after = repo.worker_profile("identity", "worker-001").unwrap();
    assert_eq!(after.worker.name, before.worker.name);
    assert_eq!(after.worker.attributes, before.worker.attributes);
    assert_eq!(after.worker.condition, before.worker.condition);
    assert!(
        after
            .worker
            .identity
            .personality
            .values()
            .all(|a| a.value.is_none())
    );
    assert_eq!(
        after.worker.identity.languages[0].name,
        before.worker.language
    );
    assert!(after.worker.identity.languages[0].proficiency.is_none());
    let backup = Connection::open(dir.path().join("identity.sqlite3.v4.bak")).unwrap();
    assert_eq!(
        backup
            .pragma_query_value::<u32, _>(None, "user_version", |r| r.get(0))
            .unwrap(),
        4
    );
    assert_eq!(repo.load_game("identity").unwrap(), updated);
}

#[test]
fn schema_five_checkpoint_remains_compatible_when_schema_six_is_interrupted() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let path = dir.path().join("identity.sqlite3");
    let db = Connection::open(&path).unwrap();
    downgrade_v4(&db);
    db.execute_batch("CREATE TABLE personal_relationships(blocker INTEGER) STRICT;")
        .unwrap();
    drop(db);

    assert!(repo.load_game("identity").is_err());
    let db = Connection::open(&path).unwrap();
    assert_eq!(
        db.pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        5
    );
    assert_eq!(
        db.query_row::<String, _, _>(
            "SELECT value FROM metadata WHERE key='engine_version'",
            [],
            |row| row.get(0),
        )
        .unwrap(),
        "0.4.0"
    );
    drop(db);
    assert_eq!(repo.list_saves().unwrap().len(), 1);

    let db = Connection::open(&path).unwrap();
    db.execute_batch("DROP TABLE personal_relationships;")
        .unwrap();
    drop(db);
    let upgraded = repo.load_game("identity").unwrap();
    assert_eq!(upgraded.schema_version, 8);
    assert_eq!(upgraded.engine_version, "0.7.0");
}

#[test]
fn invalid_paused_snapshot_rolls_back_schema_four_upgrade() {
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    downgrade_v4(&db);
    db.execute(
        "INSERT INTO show_runtime(show_id,snapshot) VALUES(1,?1)",
        params!["{\"workers\":[{\"id\":\"missing\"}]}"],
    )
    .unwrap();
    drop(db);
    assert!(repo.load_game("identity").is_err());
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    assert_eq!(
        db.pragma_query_value::<u32, _>(None, "user_version", |r| r.get(0))
            .unwrap(),
        4
    );
    assert_eq!(
        db.query_row::<String, _, _>(
            "SELECT value FROM metadata WHERE key='engine_version'",
            [],
            |r| r.get(0)
        )
        .unwrap(),
        "0.3.0"
    );
}

#[test]
fn schema_four_paused_show_resumes_without_changing_match_output() {
    use wm_domain::game::*;
    let dir = tempdir().unwrap();
    let repo = create(dir.path());
    let office = repo.career_office("identity").unwrap();
    let roster = repo.roster_page("identity", "", 0, 64).unwrap();
    let a = &roster.rows[0];
    let b = &roster.rows[1];
    let card = repo
        .save_segment(SaveSegmentRequest {
            save_id: "identity".into(),
            show_id: office.show.id,
            revision: office.show.revision,
            segment_id: None,
            content: SegmentPlan::Match(MatchPlan {
                match_type: "Singles".into(),
                worker_a: a.id.clone(),
                worker_b: b.id.clone(),
                winner_id: Some(a.id.clone()),
                finish: Finish::Pinfall,
                clean_finish: true,
                duration_seconds: 600,
                style: a.style.clone(),
                pace: 3,
                risk: 2,
                freedom: 60,
                purpose: "Competitive".into(),
                protected_worker_id: None,
                agent_id: office.agents[0].id,
                beats: vec![],
            }),
        })
        .unwrap();
    let started = repo.start_show("identity", card.id).unwrap();
    let partial = repo
        .advance_show(AdvanceRequest {
            save_id: "identity".into(),
            show_id: card.id,
            expected_tick: started.tick,
            seconds: 123,
        })
        .unwrap();
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    let json: String = db
        .query_row(
            "SELECT snapshot FROM show_runtime WHERE show_id=?1",
            [card.id],
            |r| r.get(0),
        )
        .unwrap();
    let mut session: wm_sim::runtime::Session = serde_json::from_str(&json).unwrap();
    session.advance(3600);
    let expected = serde_json::to_value(&session).unwrap();
    let mut snapshot: serde_json::Value = serde_json::from_str(&json).unwrap();
    for worker in snapshot["workers"].as_array_mut().unwrap() {
        worker.as_object_mut().unwrap().remove("identity");
    }
    db.execute(
        "UPDATE show_runtime SET snapshot=?1",
        [snapshot.to_string()],
    )
    .unwrap();
    downgrade_v4(&db);
    drop(db);
    repo.load_game("identity").unwrap();
    let completed = repo
        .advance_show(AdvanceRequest {
            save_id: "identity".into(),
            show_id: card.id,
            expected_tick: partial.tick,
            seconds: 3600,
        })
        .unwrap();
    assert!(completed.complete);
    let db = Connection::open(dir.path().join("identity.sqlite3")).unwrap();
    let json: String = db
        .query_row(
            "SELECT snapshot FROM show_runtime WHERE show_id=?1",
            [card.id],
            |r| r.get(0),
        )
        .unwrap();
    let mut actual: serde_json::Value = serde_json::from_str(&json).unwrap();
    let mut expected = expected;
    for state in [&mut actual, &mut expected] {
        for worker in state["workers"].as_array_mut().unwrap() {
            worker.as_object_mut().unwrap().remove("identity");
        }
    }
    assert_eq!(actual, expected);
    assert!(
        repo.worker_profile("identity", &a.id)
            .unwrap()
            .worker
            .identity
            .personality
            .values()
            .all(|v| v.value.is_none())
    );
}
