use super::*;
use rusqlite::{Transaction, TransactionBehavior};
use serde_json::Value;
use wm_domain::game::Worker;
use wm_domain::ratings::{LegacyAttributes, WrestlerAttributes, WrestlingStyleProfile};

const MIGRATION_V4: &str = include_str!("../migrations/004_wrestler_ratings.sql");

fn has_wrestling_style_column(tx: &Transaction<'_>) -> Result<bool, PersistenceError> {
    let mut statement = tx.prepare("PRAGMA table_info(workers)")?;
    let names = statement
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(names.iter().any(|name| name == "wrestling_style"))
}

fn migrate_worker_value(worker: &mut Value) -> Result<(), PersistenceError> {
    let source_style = worker
        .get("style")
        .and_then(Value::as_str)
        .ok_or(PersistenceError::InvalidDatabase)?;
    if worker.get("wrestlingStyle").is_some() {
        let parsed: Worker = serde_json::from_value(worker.clone())
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        parsed
            .wrestling_style
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        return Ok(());
    }
    let legacy: LegacyAttributes = serde_json::from_value(
        worker
            .get("attributes")
            .cloned()
            .ok_or(PersistenceError::InvalidDatabase)?,
    )
    .map_err(|_| PersistenceError::InvalidDatabase)?;
    let attributes = WrestlerAttributes::from_legacy(&legacy, source_style);
    let style = WrestlingStyleProfile::from_legacy(&legacy, source_style, &attributes);
    worker["attributes"] =
        serde_json::to_value(attributes).map_err(|_| PersistenceError::InvalidDatabase)?;
    worker["wrestlingStyle"] =
        serde_json::to_value(style).map_err(|_| PersistenceError::InvalidDatabase)?;
    Ok(())
}

fn migrate_live_sessions(tx: &Transaction<'_>) -> Result<(), PersistenceError> {
    let mut statement = tx.prepare("SELECT show_id,snapshot FROM show_runtime ORDER BY show_id")?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(statement);
    for (show_id, json) in rows {
        let mut session: Value =
            serde_json::from_str(&json).map_err(|_| PersistenceError::InvalidDatabase)?;
        let workers = session
            .get_mut("workers")
            .and_then(Value::as_array_mut)
            .ok_or(PersistenceError::InvalidDatabase)?;
        for worker in workers {
            migrate_worker_value(worker)?;
        }
        tx.execute(
            "UPDATE show_runtime SET snapshot=?1 WHERE show_id=?2",
            params![
                serde_json::to_string(&session).map_err(|_| PersistenceError::InvalidDatabase)?,
                show_id
            ],
        )?;
    }
    Ok(())
}

pub(super) fn migrate(
    connection: &mut Connection,
    record_upgrade: bool,
) -> Result<(), PersistenceError> {
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let version: u32 = tx.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version >= 4 {
        return Ok(());
    }
    if version != 3 {
        return Err(PersistenceError::InvalidDatabase);
    }
    if !has_wrestling_style_column(&tx)? {
        tx.execute_batch(MIGRATION_V4)?;
    }

    let mut statement =
        tx.prepare("SELECT id,style,attributes,wrestling_style FROM workers ORDER BY id")?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(statement);

    for (id, source_style, attributes_json, style_json) in rows {
        let (attributes, style) = match (
            serde_json::from_str::<WrestlerAttributes>(&attributes_json),
            style_json
                .as_deref()
                .map(serde_json::from_str::<WrestlingStyleProfile>),
        ) {
            (Ok(attributes), Some(Ok(style))) => {
                style
                    .validate()
                    .map_err(|_| PersistenceError::InvalidDatabase)?;
                (attributes, style)
            }
            (Err(_), None) => {
                let legacy: LegacyAttributes = serde_json::from_str(&attributes_json)
                    .map_err(|_| PersistenceError::InvalidDatabase)?;
                let attributes = WrestlerAttributes::from_legacy(&legacy, &source_style);
                let style = WrestlingStyleProfile::from_legacy(&legacy, &source_style, &attributes);
                (attributes, style)
            }
            _ => return Err(PersistenceError::InvalidDatabase),
        };
        tx.execute(
            "UPDATE workers SET attributes=?1,wrestling_style=?2 WHERE id=?3",
            params![
                serde_json::to_string(&attributes)
                    .map_err(|_| PersistenceError::InvalidDatabase)?,
                serde_json::to_string(&style).map_err(|_| PersistenceError::InvalidDatabase)?,
                id
            ],
        )?;
    }
    migrate_live_sessions(&tx)?;
    let invalid: i32 = tx.query_row(
        "SELECT COUNT(*) FROM workers WHERE wrestling_style IS NULL OR NOT json_valid(attributes) OR NOT json_valid(wrestling_style)",
        [],
        |row| row.get(0),
    )?;
    if invalid != 0 {
        return Err(PersistenceError::InvalidDatabase);
    }
    tx.execute(
        "UPDATE metadata SET value=?1 WHERE key='engine_version'",
        ["0.3.0"],
    )?;
    tx.execute(
        "UPDATE metadata SET value='4' WHERE key='schema_version'",
        [],
    )?;
    tx.pragma_update(None, "user_version", 4)?;
    if record_upgrade {
        let date: String =
            tx.query_row("SELECT current_date FROM promotions LIMIT 1", [], |row| {
                row.get(0)
            })?;
        tx.execute(
            "INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('save_upgraded',?1,'{\"schemaVersion\":4}')",
            [&date],
        )?;
    }
    tx.commit()?;
    Ok(())
}
