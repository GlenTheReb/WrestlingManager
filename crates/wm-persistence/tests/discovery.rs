use rusqlite::Connection;
use tempfile::tempdir;
use wm_domain::CreateGameRequest;
use wm_domain::discovery::{BlacklistMode, WorkerSearchRequest};
use wm_persistence::SaveRepository;

fn repository() -> (tempfile::TempDir, SaveRepository) {
    let directory = tempdir().unwrap();
    let repo = SaveRepository::new(directory.path());
    repo.create_game(CreateGameRequest {
        save_id: "finder-test".into(),
        seed: "2319".into(),
    })
    .unwrap();
    (directory, repo)
}

#[test]
fn worker_search_filters_sorts_and_pages_from_the_discovery_index() {
    let (_directory, repo) = repository();
    let first = repo
        .worker_search("finder-test", WorkerSearchRequest::default())
        .unwrap();
    assert_eq!(first.rows.len(), first.total.min(50));
    assert!(first.total >= first.rows.len());
    assert!(
        first.rows.windows(2).all(|pair| {
            pair[0].worker.name.to_lowercase() <= pair[1].worker.name.to_lowercase()
        })
    );

    let target = first.rows[0].worker.clone();
    let mut filtered = WorkerSearchRequest::default();
    filtered.filters.overall.minimum = Some(target.overall);
    filtered.filters.overall.maximum = Some(target.overall);
    filtered.limit = 100;
    let result = repo.worker_search("finder-test", filtered).unwrap();
    assert!(!result.rows.is_empty());
    assert!(
        result
            .rows
            .iter()
            .all(|hit| hit.worker.overall == target.overall)
    );
}

#[test]
fn saved_views_shortlists_and_blacklist_survive_a_repository_restart() {
    let (directory, repo) = repository();
    let worker = repo
        .worker_search("finder-test", WorkerSearchRequest::default())
        .unwrap()
        .rows[0]
        .worker
        .clone();
    let lists = repo
        .create_worker_shortlist("finder-test", "Main-event prospects")
        .unwrap();
    let shortlist = lists.shortlists[0].id;
    repo.set_worker_shortlist_member("finder-test", shortlist, &worker.id, true)
        .unwrap();
    repo.set_worker_blacklisted("finder-test", &worker.id, true)
        .unwrap();
    let mut saved = WorkerSearchRequest::default();
    saved.filters.shortlist_id = Some(shortlist);
    repo.save_worker_view(
        "finder-test",
        None,
        "Prospects",
        saved.clone(),
        vec!["name".into(), "overall".into()],
    )
    .unwrap();

    let restarted = SaveRepository::new(directory.path());
    let restored = restarted.worker_discovery_lists("finder-test").unwrap();
    assert_eq!(restored.saved_views[0].name, "Prospects");
    assert_eq!(restored.saved_views[0].columns, ["name", "overall"]);
    assert_eq!(restored.shortlists[0].member_count, 1);
    assert_eq!(restored.blacklist_count, 1);
    let shortlisted = restarted.worker_search("finder-test", saved).unwrap();
    assert_eq!(shortlisted.total, 1);
    assert_eq!(shortlisted.rows[0].worker.id, worker.id);
    let mut blacklisted = WorkerSearchRequest::default();
    blacklisted.filters.blacklist = BlacklistMode::Only;
    assert_eq!(
        restarted
            .worker_search("finder-test", blacklisted)
            .unwrap()
            .rows[0]
            .worker
            .id,
        worker.id
    );
}

#[test]
fn close_name_matching_explains_why_a_worker_was_returned() {
    let (_directory, repo) = repository();
    let worker = repo
        .worker_search("finder-test", WorkerSearchRequest::default())
        .unwrap()
        .rows[0]
        .worker
        .clone();
    let word = worker.name.split_whitespace().next().unwrap();
    if word.chars().count() < 3 {
        return;
    }
    let mut typo = word.chars().collect::<Vec<_>>();
    typo[0] = if typo[0] == 'x' { 'z' } else { 'x' };
    let request = WorkerSearchRequest {
        text: typo.into_iter().collect(),
        ..WorkerSearchRequest::default()
    };
    let result = repo.worker_search("finder-test", request).unwrap();
    let hit = result
        .rows
        .iter()
        .find(|hit| hit.worker.id == worker.id)
        .unwrap();
    assert_eq!(hit.match_reason.as_deref(), Some("Close ring-name match"));
}

#[test]
fn categorical_filters_support_explicit_inclusion_and_exclusion() {
    let (_directory, repo) = repository();
    let nationalities = repo
        .worker_filter_options("finder-test")
        .unwrap()
        .nationalities;
    assert!(nationalities.len() >= 2);

    let all_workers = repo
        .worker_search("finder-test", WorkerSearchRequest::default())
        .unwrap()
        .total;

    let mut included_first = WorkerSearchRequest::default();
    included_first.filters.nationalities = vec![nationalities[0].clone()];
    let first_nationality_count = repo
        .worker_search("finder-test", included_first)
        .unwrap()
        .total;
    assert!(first_nationality_count > 0);

    let mut excluded = WorkerSearchRequest::default();
    excluded.filters.excluded_nationalities = vec![nationalities[0].clone()];
    excluded.limit = 100;
    let without_nationality = repo.worker_search("finder-test", excluded).unwrap();
    assert!(!without_nationality.rows.is_empty());
    assert_eq!(
        without_nationality.total,
        all_workers - first_nationality_count
    );

    let mut included = WorkerSearchRequest::default();
    included.filters.nationalities = vec![nationalities[1].clone()];
    included.limit = 100;
    let only_nationality = repo.worker_search("finder-test", included).unwrap();
    assert!(!only_nationality.rows.is_empty());

    let mut contradictory = WorkerSearchRequest::default();
    contradictory.filters.nationalities = vec![nationalities[0].clone()];
    contradictory.filters.excluded_nationalities = vec![nationalities[0].clone()];
    assert!(repo.worker_search("finder-test", contradictory).is_err());
}

#[test]
fn opening_an_earlier_schema_seven_candidate_repairs_missing_query_indexes() {
    let (directory, repo) = repository();
    let path = directory.path().join("finder-test.sqlite3");
    let connection = Connection::open(&path).unwrap();
    connection
        .execute("DROP INDEX worker_discovery_discipline", [])
        .unwrap();
    drop(connection);

    repo.worker_search("finder-test", WorkerSearchRequest::default())
        .unwrap();

    let repaired = Connection::open(path).unwrap();
    assert_eq!(
        repaired
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='worker_discovery_discipline'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap(),
        1
    );
}
