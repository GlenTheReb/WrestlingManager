use rusqlite::Connection;
use std::fs;
use tempfile::tempdir;
use wm_domain::{CURRENT_ENGINE_VERSION, CURRENT_SCHEMA_VERSION, CreateGameRequest};
use wm_persistence::{PersistenceError, SaveRepository};

fn request(save_id: &str, seed: &str) -> CreateGameRequest {
    CreateGameRequest {
        save_id: save_id.to_owned(),
        seed: seed.to_owned(),
    }
}

#[test]
fn creates_complete_game_and_reopens_it_with_an_independent_repository() {
    let temporary = tempdir().unwrap();
    let first_repository = SaveRepository::new(temporary.path());
    let created = first_repository
        .create_game(request("first-career", "18446744073709551615"))
        .unwrap();

    assert_eq!(created.save_id, "first-career");
    assert_eq!(created.promotion_id, "uwf");
    assert_eq!(created.name, "Ultimate Wrestling Federation");
    assert_eq!(created.initials, "UWF");
    assert_eq!(created.current_date, "2026-01-01");
    assert_eq!(created.cash_pence, 25_000_000);
    assert_eq!(created.seed, "18446744073709551615");
    assert_eq!(created.engine_version, CURRENT_ENGINE_VERSION);
    assert_eq!(created.schema_version, CURRENT_SCHEMA_VERSION);

    drop(first_repository);
    let reopened = SaveRepository::new(temporary.path())
        .load_game("first-career")
        .unwrap();
    assert_eq!(reopened, created);

    let independent = Connection::open(temporary.path().join("first-career.sqlite3")).unwrap();
    let event_count: i64 = independent
        .query_row("SELECT COUNT(*) FROM domain_events", [], |row| row.get(0))
        .unwrap();
    assert_eq!(event_count, 1);
}

#[test]
fn rejects_a_structurally_invalid_wrestling_style_at_the_read_boundary() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    repository
        .create_game(request("invalid-style", "17"))
        .unwrap();
    let worker_id = repository
        .roster_page("invalid-style", "", 0, 1)
        .unwrap()
        .rows[0]
        .id
        .clone();

    let connection = Connection::open(temporary.path().join("invalid-style.sqlite3")).unwrap();
    connection
        .execute(
            "UPDATE workers SET wrestling_style=json_insert(wrestling_style,'$.evidence[#]',json_extract(wrestling_style,'$.evidence[0]')) WHERE id=?1",
            [&worker_id],
        )
        .unwrap();
    drop(connection);

    assert!(matches!(
        repository.worker_profile("invalid-style", &worker_id),
        Err(PersistenceError::InvalidDatabase)
    ));
}

#[test]
fn duplicate_creation_never_overwrites_the_existing_save() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    repository.create_game(request("protected", "7")).unwrap();
    let before = fs::read(temporary.path().join("protected.sqlite3")).unwrap();

    let error = repository
        .create_game(request("protected", "8"))
        .unwrap_err();
    assert!(matches!(error, PersistenceError::AlreadyExists));
    assert_eq!(
        fs::read(temporary.path().join("protected.sqlite3")).unwrap(),
        before
    );
    assert_eq!(repository.load_game("protected").unwrap().seed, "7");
}

#[test]
fn invalid_identifiers_and_seeds_have_no_filesystem_side_effect() {
    let temporary = tempdir().unwrap();
    let saves = temporary.path().join("not-created");
    let repository = SaveRepository::new(&saves);

    for bad_id in ["../escape", "-leading", "Upper"] {
        assert!(matches!(
            repository.create_game(request(bad_id, "1")),
            Err(PersistenceError::InvalidInput(_))
        ));
    }
    for bad_seed in ["01", "-1", "18446744073709551616"] {
        assert!(matches!(
            repository.create_game(request("valid", bad_seed)),
            Err(PersistenceError::InvalidInput(_))
        ));
    }

    assert!(!saves.exists());
}

#[test]
fn loading_a_missing_save_does_not_create_any_file_or_directory() {
    let temporary = tempdir().unwrap();
    let saves = temporary.path().join("missing");
    let repository = SaveRepository::new(&saves);

    assert!(matches!(
        repository.load_game("absent"),
        Err(PersistenceError::NotFound)
    ));
    assert!(!saves.exists());
}

#[test]
fn rejects_a_non_game_database_without_modifying_it() {
    let temporary = tempdir().unwrap();
    let path = temporary.path().join("foreign.sqlite3");
    let connection = Connection::open(&path).unwrap();
    connection
        .execute("CREATE TABLE unrelated (id INTEGER)", [])
        .unwrap();
    drop(connection);
    let before = fs::read(&path).unwrap();

    let repository = SaveRepository::new(temporary.path());
    assert!(matches!(
        repository.load_game("foreign"),
        Err(PersistenceError::InvalidDatabase)
    ));
    assert_eq!(fs::read(path).unwrap(), before);
}

#[test]
fn rejects_future_schema_and_engine_versions_without_modifying_them() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    repository
        .create_game(request("future-schema", "1"))
        .unwrap();
    repository
        .create_game(request("future-engine", "2"))
        .unwrap();

    let schema_path = temporary.path().join("future-schema.sqlite3");
    let connection = Connection::open(&schema_path).unwrap();
    connection
        .pragma_update(None, "user_version", 99_u32)
        .unwrap();
    drop(connection);
    let schema_before = fs::read(&schema_path).unwrap();

    assert!(matches!(
        repository.load_game("future-schema"),
        Err(PersistenceError::UnsupportedSchema(99))
    ));
    assert_eq!(fs::read(&schema_path).unwrap(), schema_before);

    let engine_path = temporary.path().join("future-engine.sqlite3");
    let connection = Connection::open(&engine_path).unwrap();
    connection
        .execute(
            "UPDATE metadata SET value = '99.0.0' WHERE key = 'engine_version'",
            [],
        )
        .unwrap();
    drop(connection);
    let engine_before = fs::read(&engine_path).unwrap();

    assert!(matches!(
        repository.load_game("future-engine"),
        Err(PersistenceError::UnsupportedEngine)
    ));
    assert_eq!(fs::read(&engine_path).unwrap(), engine_before);
}

#[test]
fn rejects_a_save_file_renamed_to_a_different_valid_id() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    repository.create_game(request("original", "4")).unwrap();
    fs::rename(
        temporary.path().join("original.sqlite3"),
        temporary.path().join("renamed.sqlite3"),
    )
    .unwrap();

    assert!(matches!(
        repository.load_game("renamed"),
        Err(PersistenceError::InvalidDatabase)
    ));
}

#[test]
fn rejects_malformed_calendar_dates_before_returning_a_dto() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    repository.create_game(request("bad-date", "5")).unwrap();
    let path = temporary.path().join("bad-date.sqlite3");
    let connection = Connection::open(path).unwrap();
    connection
        .execute("UPDATE promotions SET current_date = '2026-02-30'", [])
        .unwrap();
    drop(connection);

    assert!(matches!(
        repository.load_game("bad-date"),
        Err(PersistenceError::InvalidDatabase)
    ));
}

#[test]
fn listing_is_sorted_paginated_and_capped() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    for (id, seed) in [("charlie", "3"), ("alpha", "1"), ("bravo", "2")] {
        repository.create_game(request(id, seed)).unwrap();
    }

    let page = repository.list_saves_page(1, 2).unwrap();
    assert_eq!(
        page.iter()
            .map(|save| save.save_id.as_str())
            .collect::<Vec<_>>(),
        ["bravo", "charlie"]
    );
    assert_eq!(repository.list_saves().unwrap().len(), 3);
}

#[test]
fn ipc_errors_do_not_disclose_storage_paths_or_sql_details() {
    let temporary = tempdir().unwrap();
    let repository = SaveRepository::new(temporary.path());
    let error = repository.load_game("absent").unwrap_err().to_ipc_error();

    assert_eq!(error.code, "save_not_found");
    assert!(
        !error
            .message
            .contains(temporary.path().to_string_lossy().as_ref())
    );
    assert!(!error.message.to_ascii_lowercase().contains("sqlite"));
}
