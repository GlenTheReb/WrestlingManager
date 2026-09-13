use rusqlite::{Connection, params};
use tempfile::tempdir;
use wm_domain::{CreateGameRequest, characters::*, discovery::WorkerSearchRequest};
use wm_persistence::SaveRepository;

fn repository() -> (tempfile::TempDir, SaveRepository) {
    let directory = tempdir().unwrap();
    let repo = SaveRepository::new(directory.path());
    repo.create_game(CreateGameRequest {
        save_id: "characters".into(),
        seed: "4818620".into(),
    })
    .unwrap();
    (directory, repo)
}

#[test]
fn generated_people_start_with_a_public_ring_identity_and_private_profile_name() {
    let (_directory, repo) = repository();
    let worker = repo
        .roster_page("characters", "", 0, 1)
        .unwrap()
        .rows
        .remove(0);
    let profile = repo.worker_profile("characters", &worker.id).unwrap();
    assert_eq!(profile.character.active.ring_name, worker.name);
    assert_eq!(
        profile.character.legal_name.as_deref(),
        Some(worker.name.as_str())
    );
    assert_eq!(
        profile.character.active.identity_knowledge,
        IdentityKnowledge::Public
    );
    assert!(profile.character.history.is_empty());
}

#[test]
fn accepted_change_waits_for_public_launch_preserves_stats_and_searches_old_alias() {
    let (_directory, repo) = repository();
    let worker = repo
        .roster_page("characters", "", 0, 64)
        .unwrap()
        .rows
        .into_iter()
        .find(|w| w.condition.morale >= 55)
        .unwrap();
    let before = repo.worker_profile("characters", &worker.id).unwrap();
    let mut gimmick = before.character.active.gimmick.clone();
    gimmick.name = "Midnight Vanguard".into();
    gimmick.description = "A masked risk-taker who fights from underneath.".into();
    gimmick.tags = vec!["masked".into(), "resilient".into()];
    let request = ProposeCharacterChangeRequest {
        save_id: "characters".into(),
        worker_id: worker.id.clone(),
        request_id: "proposal-1".into(),
        expected_revision: 0,
        ring_name: "The Midnight Vanguard".into(),
        alignment_intent: AlignmentIntent::Face,
        masked: true,
        concealed: true,
        intended_launch_on: Some("2026-01-01".into()),
        gimmick,
    };
    let planned = repo.propose_character_change(request.clone()).unwrap();
    assert_eq!(planned.active.ring_name, before.character.active.ring_name);
    assert_eq!(
        repo.propose_character_change(request)
            .unwrap()
            .pending_change
            .unwrap()
            .id,
        planned.pending_change.as_ref().unwrap().id
    );
    let change = planned.pending_change.unwrap();
    let launched = repo
        .launch_character_change(CharacterActionRequest {
            save_id: "characters".into(),
            worker_id: worker.id.clone(),
            request_id: "launch-1".into(),
            change_id: Some(change.id),
            expected_revision: change.revision,
        })
        .unwrap();
    assert_eq!(launched.active.ring_name, "The Midnight Vanguard");
    assert_eq!(
        launched.history[0].ring_name,
        before.character.active.ring_name
    );
    let after = repo.worker_profile("characters", &worker.id).unwrap();
    assert_eq!(after.worker.attributes, before.worker.attributes);
    assert_eq!(after.wrestling, before.wrestling);
    let search = WorkerSearchRequest {
        text: before.worker.name,
        limit: 10,
        ..WorkerSearchRequest::default()
    };
    let result = repo.worker_search("characters", search).unwrap();
    assert_eq!(result.rows[0].worker.name, "The Midnight Vanguard");
    assert_eq!(
        result.rows[0].match_reason.as_deref(),
        Some("Matched former ring name")
    );
}

#[test]
fn character_retirement_does_not_remove_the_person_or_history() {
    let (_directory, repo) = repository();
    let worker = repo
        .roster_page("characters", "", 0, 1)
        .unwrap()
        .rows
        .remove(0);
    let retired = repo
        .set_character_retired(
            CharacterActionRequest {
                save_id: "characters".into(),
                worker_id: worker.id.clone(),
                request_id: "retire-1".into(),
                change_id: None,
                expected_revision: 1,
            },
            true,
        )
        .unwrap();
    assert_eq!(retired.active.status, CharacterStatus::Retired);
    assert_eq!(
        repo.worker_profile("characters", &worker.id)
            .unwrap()
            .worker
            .id,
        worker.id
    );
    let revived = repo
        .set_character_retired(
            CharacterActionRequest {
                save_id: "characters".into(),
                worker_id: worker.id,
                request_id: "revive-1".into(),
                change_id: None,
                expected_revision: 2,
            },
            false,
        )
        .unwrap();
    assert_eq!(revived.active.status, CharacterStatus::Active);
    assert_eq!(revived.active.revision, 3);
}

#[test]
fn retirement_rejects_stale_revisions_and_cross_action_request_reuse() {
    let (_directory, repo) = repository();
    let worker = repo
        .roster_page("characters", "", 0, 1)
        .unwrap()
        .rows
        .remove(0);
    let stale = repo.set_character_retired(
        CharacterActionRequest {
            save_id: "characters".into(),
            worker_id: worker.id.clone(),
            request_id: "status-stale".into(),
            change_id: None,
            expected_revision: 0,
        },
        true,
    );
    assert!(stale.is_err());

    repo.set_character_retired(
        CharacterActionRequest {
            save_id: "characters".into(),
            worker_id: worker.id.clone(),
            request_id: "status-change".into(),
            change_id: None,
            expected_revision: 1,
        },
        true,
    )
    .unwrap();
    let conflicting_replay = repo.set_character_retired(
        CharacterActionRequest {
            save_id: "characters".into(),
            worker_id: worker.id,
            request_id: "status-change".into(),
            change_id: None,
            expected_revision: 2,
        },
        false,
    );
    assert!(conflicting_replay.is_err());
}

#[test]
fn profile_prefers_the_player_company_and_database_rejects_context_collisions() {
    let (directory, repo) = repository();
    let worker = repo
        .roster_page("characters", "", 0, 1)
        .unwrap()
        .rows
        .remove(0);
    let before = repo.worker_profile("characters", &worker.id).unwrap();
    let database = Connection::open(directory.path().join("characters.sqlite3")).unwrap();
    let gimmick = serde_json::to_string(&GimmickBrief::default()).unwrap();
    database
        .execute(
            "INSERT INTO characters(id,worker_id,ring_name,alignment_intent,status,masked,concealed,gimmick,created_on) VALUES('other-character',?1,'El Relámpago','face','active',1,1,?2,'2026-01-01')",
            params![worker.id, gimmick],
        )
        .unwrap();
    database
        .execute(
            "INSERT INTO character_tenures(character_id,company_id,brand,started_on,ended_on,knowledge) VALUES('other-character','other-company',NULL,'2026-01-01',NULL,'rumoured')",
            [],
        )
        .unwrap();

    let profile = repo.worker_profile("characters", &worker.id).unwrap();
    assert_eq!(profile.character.active.id, before.character.active.id);
    assert!(
        profile
            .character
            .history
            .iter()
            .any(|identity| identity.id == "other-character" && identity.ended_on.is_none())
    );

    database
        .execute(
            "INSERT INTO characters(id,worker_id,ring_name,alignment_intent,status,masked,concealed,gimmick,created_on) VALUES('collision-character',?1,'Another Name','heel','active',0,0,?2,'2026-01-01')",
            params![worker.id, gimmick],
        )
        .unwrap();
    let collision = database.execute(
        "INSERT INTO character_tenures(character_id,company_id,brand,started_on,ended_on,knowledge) VALUES('collision-character',?1,NULL,'2026-01-01',NULL,'public')",
        [before.character.active.company_id],
    );
    assert!(collision.is_err());
}
