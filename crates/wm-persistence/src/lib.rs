//! SQLite persistence for portable Wrestling Manager save files.

mod career;
mod news;

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::Builder;
use thiserror::Error;
use wm_domain::{
    CURRENT_ENGINE_VERSION, CURRENT_SCHEMA_VERSION, CreateGameRequest, IpcError, PromotionOverview,
    SaveId, SaveSummary, ValidationError,
};

const APPLICATION_ID: u32 = 0x574D_4752;
const DEFAULT_LIST_LIMIT: usize = 100;
const MAX_LIST_LIMIT: usize = 100;
const SAVE_EXTENSION: &str = "sqlite3";

const INITIAL_PROMOTION_ID: &str = "uwf";
const INITIAL_PROMOTION_NAME: &str = "Ultimate Wrestling Federation";
const INITIAL_PROMOTION_INITIALS: &str = "UWF";
const INITIAL_REGION: &str = "United Kingdom";
// The walking skeleton starts from the fixed date defined by the SDD.
const INITIAL_DATE: &str = "2026-01-01";
const INITIAL_CASH_PENCE: i64 = 25_000_000;
const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;

const MIGRATION_V1: &str = r#"
CREATE TABLE metadata (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) STRICT;

CREATE TABLE promotions (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    initials TEXT NOT NULL,
    region TEXT NOT NULL,
    founded_on TEXT NOT NULL,
    current_date TEXT NOT NULL,
    cash_pence INTEGER NOT NULL CHECK (cash_pence BETWEEN 0 AND 9007199254740991)
) STRICT;

CREATE TABLE domain_events (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    occurred_on TEXT NOT NULL,
    payload TEXT NOT NULL
) STRICT;
"#;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("{0}")]
    GameRule(String),
    #[error(transparent)]
    InvalidInput(#[from] ValidationError),
    #[error("a save with this ID already exists")]
    AlreadyExists,
    #[error("the requested save does not exist")]
    NotFound,
    #[error("the file is not a Wrestling Manager save")]
    InvalidDatabase,
    #[error("save schema version {0} is not supported")]
    UnsupportedSchema(u32),
    #[error("save engine version is not supported")]
    UnsupportedEngine,
    #[error("save storage operation failed")]
    Storage(#[source] rusqlite::Error),
    #[error("save filesystem operation failed")]
    Filesystem(#[source] std::io::Error),
}

impl PersistenceError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::GameRule(_) => "game_rule",
            Self::InvalidInput(_) => "invalid_input",
            Self::AlreadyExists => "save_already_exists",
            Self::NotFound => "save_not_found",
            Self::InvalidDatabase => "invalid_save",
            Self::UnsupportedSchema(_) => "unsupported_schema_version",
            Self::UnsupportedEngine => "unsupported_engine_version",
            Self::Storage(_) | Self::Filesystem(_) => "storage_error",
        }
    }

    pub fn to_ipc_error(&self) -> IpcError {
        IpcError {
            code: self.code().to_owned(),
            message: self.to_string(),
        }
    }
}

impl From<rusqlite::Error> for PersistenceError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Storage(error)
    }
}

#[derive(Debug, Clone)]
pub struct SaveRepository {
    saves_directory: PathBuf,
}

impl SaveRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            saves_directory: path.into(),
        }
    }

    pub fn create_game(
        &self,
        request: CreateGameRequest,
    ) -> Result<PromotionOverview, PersistenceError> {
        let spec = request.validate()?;
        fs::create_dir_all(&self.saves_directory).map_err(PersistenceError::Filesystem)?;
        let path = self.path_for(&spec.save_id);
        if path.exists() {
            return Err(PersistenceError::AlreadyExists);
        }

        let temporary_path = Builder::new()
            .prefix(".creating-")
            .suffix(".sqlite3")
            .tempfile_in(&self.saves_directory)
            .map_err(PersistenceError::Filesystem)?
            .into_temp_path();
        let mut connection = Connection::open_with_flags(
            &temporary_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        initialise_v1(
            &mut connection,
            spec.save_id.as_str(),
            &spec.seed.canonical(),
        )?;
        career::initialise_gameplay(&mut connection, false)?;
        let overview = read_overview(&connection)?;
        drop(connection);

        match temporary_path.persist_noclobber(&path) {
            Ok(_) => {}
            Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(PersistenceError::AlreadyExists);
            }
            Err(error) => return Err(PersistenceError::Filesystem(error.error)),
        }
        Ok(overview)
    }

    pub fn load_game(&self, save_id: &str) -> Result<PromotionOverview, PersistenceError> {
        let save_id = SaveId::parse(save_id)?;
        let connection = self.open_existing(&save_id)?;
        validate_compatibility(&connection)?;
        let overview = read_overview(&connection)?;
        if overview.save_id != save_id.as_str() {
            return Err(PersistenceError::InvalidDatabase);
        }
        if overview.schema_version < CURRENT_SCHEMA_VERSION {
            drop(connection);
            self.upgrade_career(&save_id)?;
            return self.load_game(save_id.as_str());
        }
        Ok(overview)
    }

    pub fn get_promotion_overview(
        &self,
        save_id: &str,
    ) -> Result<PromotionOverview, PersistenceError> {
        self.load_game(save_id)
    }

    pub fn list_saves(&self) -> Result<Vec<SaveSummary>, PersistenceError> {
        self.list_saves_page(0, DEFAULT_LIST_LIMIT)
    }

    pub fn list_saves_page(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<SaveSummary>, PersistenceError> {
        let limit = limit.min(MAX_LIST_LIMIT);
        if limit == 0 || !self.saves_directory.exists() {
            return Ok(Vec::new());
        }

        let mut candidates = fs::read_dir(&self.saves_directory)
            .map_err(PersistenceError::Filesystem)?
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                if !entry.file_type().ok()?.is_file()
                    || path.extension()?.to_str()? != SAVE_EXTENSION
                {
                    return None;
                }
                let id = path.file_stem()?.to_str()?;
                SaveId::parse(id).ok().map(|save_id| (save_id, path))
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| left.0.as_str().cmp(right.0.as_str()));

        let mut summaries = Vec::with_capacity(limit);
        for (save_id, path) in candidates.into_iter().skip(offset) {
            if summaries.len() == limit {
                break;
            }
            let Ok(connection) = open_read_only(&path) else {
                continue;
            };
            if validate_compatibility(&connection).is_err() {
                continue;
            }
            if let Ok(overview) = read_overview(&connection)
                && overview.save_id == save_id.as_str()
            {
                summaries.push(SaveSummary {
                    save_id: overview.save_id,
                    promotion_name: overview.name,
                    current_date: overview.current_date,
                    seed: overview.seed,
                });
            }
        }

        Ok(summaries)
    }

    fn path_for(&self, save_id: &SaveId) -> PathBuf {
        self.saves_directory
            .join(save_id.as_str())
            .with_extension(SAVE_EXTENSION)
    }

    fn open_existing(&self, save_id: &SaveId) -> Result<Connection, PersistenceError> {
        let path = self.path_for(save_id);
        if !path.is_file() {
            return Err(PersistenceError::NotFound);
        }
        open_read_only(&path).map_err(PersistenceError::Storage)
    }
}

fn open_read_only(path: &Path) -> rusqlite::Result<Connection> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
}

fn initialise_v1(
    connection: &mut Connection,
    save_id: &str,
    seed: &str,
) -> Result<(), PersistenceError> {
    let transaction = connection.transaction()?;
    transaction.execute_batch(MIGRATION_V1)?;
    transaction.pragma_update(None, "application_id", APPLICATION_ID)?;
    transaction.pragma_update(None, "user_version", 1)?;

    for (key, value) in [
        ("game_format", "wrestling-manager"),
        ("save_id", save_id),
        ("seed", seed),
        ("engine_version", "0.1.0"),
        ("schema_version", "1"),
    ] {
        transaction.execute(
            "INSERT INTO metadata (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
    }

    transaction.execute(
        "INSERT INTO promotions
         (id, name, initials, region, founded_on, current_date, cash_pence)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            INITIAL_PROMOTION_ID,
            INITIAL_PROMOTION_NAME,
            INITIAL_PROMOTION_INITIALS,
            INITIAL_REGION,
            INITIAL_DATE,
            INITIAL_DATE,
            INITIAL_CASH_PENCE
        ],
    )?;
    transaction.execute(
        "INSERT INTO domain_events (event_type, occurred_on, payload)
         VALUES ('game_created', ?1, ?2)",
        params![
            INITIAL_DATE,
            format!("{{\"saveId\":\"{save_id}\",\"promotionId\":\"{INITIAL_PROMOTION_ID}\"}}")
        ],
    )?;
    transaction.commit()?;
    Ok(())
}

fn validate_compatibility(connection: &Connection) -> Result<(), PersistenceError> {
    let application_id: u32 =
        connection.pragma_query_value(None, "application_id", |row| row.get(0))?;
    if application_id != APPLICATION_ID {
        return Err(PersistenceError::InvalidDatabase);
    }

    let schema_version: u32 =
        connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if schema_version > CURRENT_SCHEMA_VERSION {
        return Err(PersistenceError::UnsupportedSchema(schema_version));
    }
    if !(1..=CURRENT_SCHEMA_VERSION).contains(&schema_version) {
        return Err(PersistenceError::InvalidDatabase);
    }

    let game_format = metadata_value(connection, "game_format")?;
    if game_format.as_deref() != Some("wrestling-manager") {
        return Err(PersistenceError::InvalidDatabase);
    }

    let stored_schema = metadata_value(connection, "schema_version")?;
    if stored_schema.as_deref() != Some(schema_version.to_string().as_str()) {
        return Err(PersistenceError::InvalidDatabase);
    }

    let engine_version =
        metadata_value(connection, "engine_version")?.ok_or(PersistenceError::InvalidDatabase)?;
    if engine_version
        != if schema_version == 1 {
            "0.1.0"
        } else {
            CURRENT_ENGINE_VERSION
        }
    {
        return Err(PersistenceError::UnsupportedEngine);
    }

    Ok(())
}

fn metadata_value(connection: &Connection, key: &str) -> Result<Option<String>, PersistenceError> {
    connection
        .query_row("SELECT value FROM metadata WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .optional()
        .map_err(|error| match error {
            rusqlite::Error::SqliteFailure(_, _) | rusqlite::Error::InvalidColumnType(_, _, _) => {
                PersistenceError::InvalidDatabase
            }
            other => PersistenceError::Storage(other),
        })
}

fn read_overview(connection: &Connection) -> Result<PromotionOverview, PersistenceError> {
    let save_id =
        metadata_value(connection, "save_id")?.ok_or(PersistenceError::InvalidDatabase)?;
    SaveId::parse(&save_id).map_err(|_| PersistenceError::InvalidDatabase)?;
    let seed = metadata_value(connection, "seed")?.ok_or(PersistenceError::InvalidDatabase)?;
    wm_domain::Seed::parse(&seed).map_err(|_| PersistenceError::InvalidDatabase)?;

    let overview = connection
        .query_row(
            "SELECT id, name, initials, region, founded_on, promotions.current_date, cash_pence
             FROM promotions ORDER BY id LIMIT 1",
            [],
            |row| {
                Ok(PromotionOverview {
                    save_id,
                    promotion_id: row.get(0)?,
                    name: row.get(1)?,
                    initials: row.get(2)?,
                    region: row.get(3)?,
                    founded_on: row.get(4)?,
                    current_date: row.get(5)?,
                    cash_pence: row.get(6)?,
                    seed,
                    engine_version: metadata_value(connection, "engine_version")
                        .ok()
                        .flatten()
                        .unwrap_or_default(),
                    schema_version: connection
                        .pragma_query_value(None, "user_version", |r| r.get(0))?,
                })
            },
        )
        .optional()?
        .ok_or(PersistenceError::InvalidDatabase)?;

    if !(0..=MAX_SAFE_INTEGER).contains(&overview.cash_pence) {
        return Err(PersistenceError::InvalidDatabase);
    }
    if !is_iso_date(&overview.founded_on) || !is_iso_date(&overview.current_date) {
        return Err(PersistenceError::InvalidDatabase);
    }

    Ok(overview)
}

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    if bytes
        .iter()
        .enumerate()
        .any(|(index, byte)| index != 4 && index != 7 && !byte.is_ascii_digit())
    {
        return false;
    }

    let Ok(year) = value[0..4].parse::<u32>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = value[8..10].parse::<u32>() else {
        return false;
    };
    let leap_year =
        year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days_in_month).contains(&day)
}
