use super::*;
use rusqlite::TransactionBehavior;
use wm_domain::identity::{PersonIdentity, SpokenLanguage};
use wm_domain::traits::{IdentityEvent, TraitLedger};

fn encode<T: serde::Serialize>(value: &T) -> Result<String, PersistenceError> {
    serde_json::to_string(value).map_err(|_| PersistenceError::InvalidDatabase)
}
fn parse(value: &str) -> Result<PersonIdentity, PersistenceError> {
    let identity: PersonIdentity =
        serde_json::from_str(value).map_err(|_| PersistenceError::InvalidDatabase)?;
    identity
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    Ok(identity)
}
pub(super) fn migrate(
    connection: &mut Connection,
    migration: bool,
) -> Result<(), PersistenceError> {
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let version: u32 = tx.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if version == 5 {
        return Ok(());
    }
    if version != 4 {
        return Err(PersistenceError::InvalidDatabase);
    }
    let columns = tx
        .prepare("PRAGMA table_info(workers)")?
        .query_map([], |r| r.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    if !columns.iter().any(|c| c == "identity") {
        tx.execute_batch("ALTER TABLE workers ADD COLUMN identity TEXT CHECK(identity IS NULL OR json_valid(identity));")?;
    }
    tx.execute_batch(include_str!("../migrations/005_person_identity.sql"))?;
    let rows = tx
        .prepare("SELECT id,language,identity FROM workers ORDER BY id")?
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, Option<String>>(2)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut identities = std::collections::BTreeMap::new();
    for (id, language, json) in rows {
        let identity = if let Some(json) = json {
            parse(&json)?
        } else {
            // Legacy adjectives do not justify precise private personality numbers.
            let mut identity = PersonIdentity::default();
            if !language.trim().is_empty() {
                identity.languages.push(SpokenLanguage {
                    name: language.trim().into(),
                    proficiency: None,
                    native: false,
                });
            }
            identity
                .validate()
                .map_err(|_| PersistenceError::InvalidDatabase)?;
            identity
        };
        tx.execute(
            "UPDATE workers SET identity=?1 WHERE id=?2",
            params![encode(&identity)?, id],
        )?;
        identities.insert(id, identity);
    }
    let rows = tx
        .prepare("SELECT show_id,snapshot FROM show_runtime ORDER BY show_id")?
        .query_map([], |r| Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for (show, json) in rows {
        let mut snapshot: serde_json::Value =
            serde_json::from_str(&json).map_err(|_| PersistenceError::InvalidDatabase)?;
        let workers = snapshot
            .get_mut("workers")
            .and_then(|v| v.as_array_mut())
            .ok_or(PersistenceError::InvalidDatabase)?;
        for worker in workers {
            let id = worker
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or(PersistenceError::InvalidDatabase)?;
            let identity = identities
                .get(id)
                .ok_or(PersistenceError::InvalidDatabase)?;
            worker["identity"] =
                serde_json::to_value(identity).map_err(|_| PersistenceError::InvalidDatabase)?;
        }
        let _: wm_sim::runtime::Session = serde_json::from_value(snapshot.clone())
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        tx.execute(
            "UPDATE show_runtime SET snapshot=?1 WHERE show_id=?2",
            params![encode(&snapshot)?, show],
        )?;
    }
    tx.execute(
        "UPDATE metadata SET value=?1 WHERE key='engine_version'",
        ["0.4.0"],
    )?;
    tx.execute(
        "UPDATE metadata SET value='5' WHERE key='schema_version'",
        [],
    )?;
    tx.pragma_update(None, "user_version", 5)?;
    if migration {
        tx.execute("INSERT INTO domain_events(event_type,occurred_on,payload) SELECT 'save_upgraded',current_date,'{\"schemaVersion\":5}' FROM promotions LIMIT 1",[])?;
    }
    tx.commit()?;
    Ok(())
}

pub(super) fn ledger(
    connection: &Connection,
    worker: &str,
) -> Result<TraitLedger, PersistenceError> {
    let row: Option<(u32, String)> = connection
        .query_row(
            "SELECT revision,ledger FROM identity_traits WHERE worker_id=?1",
            [worker],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    match row {
        None => Ok(TraitLedger::default()),
        Some((revision, json)) => {
            let ledger: TraitLedger =
                serde_json::from_str(&json).map_err(|_| PersistenceError::InvalidDatabase)?;
            if ledger.revision != revision
                || ledger.rule_version != wm_domain::traits::TRAIT_RULE_VERSION
            {
                return Err(PersistenceError::InvalidDatabase);
            }
            ledger
                .validate_loaded(worker)
                .map_err(|_| PersistenceError::InvalidDatabase)?;
            Ok(ledger)
        }
    }
}
fn store(
    connection: &Connection,
    worker: &str,
    ledger: &TraitLedger,
) -> Result<(), PersistenceError> {
    connection.execute("INSERT INTO identity_traits(worker_id,revision,ledger) VALUES(?1,?2,?3) ON CONFLICT(worker_id) DO UPDATE SET revision=excluded.revision,ledger=excluded.ledger",params![worker,ledger.revision,encode(ledger)?])?;
    Ok(())
}
pub(super) fn expire(connection: &Connection, as_of: &str) -> Result<(), PersistenceError> {
    let ids = connection
        .prepare("SELECT worker_id FROM identity_traits ORDER BY worker_id")?
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for id in ids {
        let mut ledger = ledger(connection, &id)?;
        ledger
            .evaluate(&id, as_of)
            .map_err(PersistenceError::GameRule)?;
        ledger.revision = ledger
            .revision
            .checked_add(1)
            .ok_or(PersistenceError::InvalidDatabase)?;
        store(connection, &id, &ledger)?;
    }
    Ok(())
}
impl SaveRepository {
    /// Internal producer boundary. No arbitrary-incident UI/IPC; future systems call after validation.
    pub fn record_identity_event(
        &self,
        save_id: &str,
        event: IdentityEvent,
        expected_revision: u32,
    ) -> Result<TraitLedger, PersistenceError> {
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let exists: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM workers WHERE id=?1)",
            [&event.person_id],
            |r| r.get(0),
        )?;
        if !exists {
            return Err(PersistenceError::GameRule("Unknown person.".into()));
        }
        let mut ledger = ledger(&tx, &event.person_id)?;
        // Exact retries are safe even if the caller's revision predates that same accepted event.
        if let Some(prior) = ledger.events.iter().find(|e| e.id == event.id) {
            if prior == &event {
                return Ok(ledger);
            } else {
                return Err(PersistenceError::GameRule(
                    "Conflicting identity event ID.".into(),
                ));
            }
        }
        if ledger.revision != expected_revision {
            return Err(PersistenceError::GameRule(
                "Identity evidence changed. Reload before recording.".into(),
            ));
        }
        let date = read_overview(&tx)?.current_date;
        ledger
            .record(event.clone(), &event.person_id, &date)
            .map_err(PersistenceError::GameRule)?;
        store(&tx, &event.person_id, &ledger)?;
        tx.commit()?;
        Ok(ledger)
    }
}
