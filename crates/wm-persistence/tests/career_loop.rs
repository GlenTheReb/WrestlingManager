use tempfile::tempdir;
use wm_domain::{CreateGameRequest, game::*};
use wm_persistence::SaveRepository;

fn legacy(directory: &std::path::Path) {
    let db = rusqlite::Connection::open(directory.join("legacy.sqlite3")).unwrap();
    db.execute_batch("PRAGMA application_id=0x574D4752; PRAGMA user_version=1;
        CREATE TABLE metadata(key TEXT PRIMARY KEY NOT NULL,value TEXT NOT NULL) STRICT;
        CREATE TABLE promotions(id TEXT PRIMARY KEY NOT NULL,name TEXT NOT NULL,initials TEXT NOT NULL,region TEXT NOT NULL,founded_on TEXT NOT NULL,current_date TEXT NOT NULL,cash_pence INTEGER NOT NULL) STRICT;
        CREATE TABLE domain_events(sequence INTEGER PRIMARY KEY AUTOINCREMENT,event_type TEXT NOT NULL,occurred_on TEXT NOT NULL,payload TEXT NOT NULL) STRICT;
        INSERT INTO metadata VALUES ('game_format','wrestling-manager'),('save_id','legacy'),('seed','18446744073709551615'),('engine_version','0.1.0'),('schema_version','1');
        INSERT INTO promotions VALUES ('uwf','My retained promotion','MRP','United Kingdom','2026-01-01','2026-02-12',12345678);
        INSERT INTO domain_events(event_type,occurred_on,payload) VALUES ('game_created','2026-01-01','{}');").unwrap();
}

#[test]
fn legacy_upgrade_keeps_identity_finances_and_an_independently_readable_backup() {
    let directory = tempdir().unwrap();
    legacy(directory.path());
    let repo = SaveRepository::new(directory.path());
    let updated = repo.load_game("legacy").unwrap();
    assert_eq!(updated.name, "My retained promotion");
    assert_eq!(updated.cash_pence, 12345678);
    assert_eq!(updated.current_date, "2026-02-12");
    assert_eq!(updated.schema_version, 7);
    assert_eq!(repo.career_office("legacy").unwrap().roster_count, 40);
    let backup_path = directory.path().join("legacy.sqlite3.v1.bak");
    let backup = rusqlite::Connection::open(&backup_path).unwrap();
    let version: i32 = backup
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(version, 1);
    let original = std::fs::read(&backup_path).unwrap();
    repo.load_game("legacy").unwrap();
    assert_eq!(std::fs::read(&backup_path).unwrap(), original);
}

#[test]
fn invalid_existing_backup_prevents_migration_without_touching_either_file() {
    let directory = tempdir().unwrap();
    legacy(directory.path());
    let path = directory.path().join("legacy.sqlite3");
    let before = std::fs::read(&path).unwrap();
    let backup = directory.path().join("legacy.sqlite3.v1.bak");
    std::fs::write(&backup, b"not a backup").unwrap();
    assert!(
        SaveRepository::new(directory.path())
            .load_game("legacy")
            .is_err()
    );
    assert_eq!(std::fs::read(path).unwrap(), before);
    assert_eq!(std::fs::read(backup).unwrap(), b"not a backup");
}

#[test]
fn interrupted_version_one_upgrade_can_be_repaired_and_retried() {
    let directory = tempdir().unwrap();
    legacy(directory.path());
    let path = directory.path().join("legacy.sqlite3");
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute_batch("CREATE TABLE news_items(blocker INTEGER) STRICT;")
        .unwrap();
    drop(db);

    let repo = SaveRepository::new(directory.path());
    assert!(repo.load_game("legacy").is_err());
    let db = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row::<String, _, _>(
            "SELECT value FROM metadata WHERE key='engine_version'",
            [],
            |row| row.get(0)
        )
        .unwrap(),
        "0.2.0"
    );
    assert_eq!(
        db.pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        2
    );
    db.execute("DROP TABLE news_items", []).unwrap();
    drop(db);

    assert_eq!(repo.load_game("legacy").unwrap().schema_version, 7);
}

#[test]
fn booking_restart_stale_requests_and_completion_are_transactional() {
    let directory = tempdir().unwrap();
    let repo = SaveRepository::new(directory.path());
    repo.create_game(CreateGameRequest {
        save_id: "loop".into(),
        seed: "42".into(),
    })
    .unwrap();
    let office = repo.career_office("loop").unwrap();
    let roster = repo.roster_page("loop", "", 0, 64).unwrap();
    assert_eq!(roster.total, 40);
    let a = &roster.rows[0];
    let b = &roster.rows[1];
    let plan = MatchPlan {
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
    };
    // Invalid core rules must fail before a card/revision mutation reaches SQLite.
    for (match_type, winner, finish) in [
        ("Tag", Some(a.id.clone()), Finish::Pinfall),
        ("Singles", Some("outsider".into()), Finish::Pinfall),
        ("Singles", Some(a.id.clone()), Finish::Draw),
        ("Singles", None, Finish::Submission),
    ] {
        let mut invalid = plan.clone();
        invalid.match_type = match_type.into();
        invalid.winner_id = winner;
        invalid.finish = finish;
        assert!(
            repo.save_segment(SaveSegmentRequest {
                save_id: "loop".into(),
                show_id: office.show.id,
                revision: office.show.revision,
                segment_id: None,
                content: SegmentPlan::Match(invalid),
            })
            .is_err()
        );
        assert_eq!(repo.career_office("loop").unwrap().show, office.show);
    }
    let card = repo
        .save_segment(SaveSegmentRequest {
            save_id: "loop".into(),
            show_id: office.show.id,
            revision: office.show.revision,
            segment_id: None,
            content: SegmentPlan::Match(plan),
        })
        .unwrap();
    assert!(repo.continue_day("loop").is_err());
    let started = repo.start_show("loop", card.id).unwrap();
    let request = AdvanceRequest {
        save_id: "loop".into(),
        show_id: card.id,
        expected_tick: started.tick,
        seconds: 123,
    };
    let partial = repo.advance_show(request.clone()).unwrap();
    assert!(repo.advance_show(request).is_err());
    drop(repo);
    // Restore the precise previous schema while preserving a live show's snapshot.
    let prior = rusqlite::Connection::open(directory.path().join("loop.sqlite3")).unwrap();
    prior.execute_batch("DROP TABLE worker_blacklist; DROP TABLE worker_shortlist_members; DROP TABLE worker_shortlists; DROP TABLE worker_saved_views; DROP TABLE worker_discovery_index; DROP TABLE player_interactions; DROP TABLE management_relationships; DROP TABLE relationship_memories; DROP TABLE personal_relationships; DROP TABLE news_items; DROP TABLE identity_traits; PRAGMA user_version=2; UPDATE metadata SET value='2' WHERE key='schema_version'; UPDATE metadata SET value='0.2.0' WHERE key='engine_version';").unwrap();
    drop(prior);
    let repo = SaveRepository::new(directory.path());
    assert_eq!(repo.live_show("loop", card.id).unwrap(), partial);
    assert!(directory.path().join("loop.sqlite3.v2.bak").exists());
    let completed = repo
        .advance_show(AdvanceRequest {
            save_id: "loop".into(),
            show_id: card.id,
            expected_tick: partial.tick,
            seconds: 3600,
        })
        .unwrap();
    assert!(completed.complete);
    assert_eq!(
        completed.report.as_ref().unwrap().segments[0]
            .winner_id
            .as_ref(),
        Some(&a.id)
    );
    let after = repo.career_office("loop").unwrap();
    let news = repo.news_page("loop", "", false, 0, 50).unwrap();
    assert_eq!(
        news.items
            .iter()
            .filter(|item| item.category == "Results")
            .count(),
        1
    );
    assert_eq!(
        news.items
            .iter()
            .filter(|item| item.category == "Business")
            .count(),
        1
    );
    let results = news
        .items
        .iter()
        .find(|item| item.category == "Results")
        .unwrap();
    assert_eq!(results.show_id, Some(card.id));
    assert!(
        results
            .body
            .contains(&completed.report.as_ref().unwrap().segments[0].result)
    );
    repo.set_news_read("loop", results.id, true).unwrap();
    assert_eq!(
        repo.news_page("loop", "", true, 0, 50).unwrap().unread,
        news.unread - 1
    );
    let profile = repo.worker_profile("loop", &a.id).unwrap();
    assert_eq!(profile.history.len(), 1);
    assert_eq!(profile.worker.condition.matches, 1);
    assert!(!after.media.is_empty());
    assert_eq!(after.show.date, "2026-01-08");
    let duplicate = repo
        .advance_show(AdvanceRequest {
            save_id: "loop".into(),
            show_id: card.id,
            expected_tick: 0,
            seconds: 3600,
        })
        .unwrap();
    assert_eq!(duplicate, completed);
    assert_eq!(repo.career_office("loop").unwrap(), after);
    assert_eq!(
        repo.news_page("loop", "", false, 0, 50).unwrap().total,
        news.total
    );
    let db = rusqlite::Connection::open(directory.path().join("loop.sqlite3")).unwrap();
    let ledger: i32 = db
        .query_row("SELECT COUNT(*) FROM ledger", [], |r| r.get(0))
        .unwrap();
    assert_eq!(ledger, 1);
    db.execute(
        "UPDATE workers SET condition=json_set(condition,'$.injuryDays',1) WHERE id=?1",
        [&a.id],
    )
    .unwrap();
    let tomorrow = repo.continue_day("loop").unwrap();
    assert_eq!(tomorrow.promotion.current_date, "2026-01-02");
    let medical = repo.news_page("loop", "Medical", false, 0, 50).unwrap();
    assert!(
        medical
            .items
            .iter()
            .any(|item| item.worker_id.as_ref() == Some(&a.id) && item.title.contains("cleared"))
    );
    drop(repo);
    let repo = SaveRepository::new(directory.path());
    assert!(
        repo.news_page("loop", "Results", false, 0, 50)
            .unwrap()
            .items[0]
            .read
    );
    assert!(
        repo.worker_profile("loop", &a.id)
            .unwrap()
            .worker
            .condition
            .fatigue
            <= profile.worker.condition.fatigue
    );
}
