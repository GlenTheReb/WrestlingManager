use super::*;
use rusqlite::functions::FunctionFlags;
use rusqlite::types::Value;
use rusqlite::{TransactionBehavior, params, params_from_iter};
use std::collections::BTreeSet;
use wm_domain::discovery::*;
use wm_domain::game::{RosterRow, Worker};

const DISCOVERY_INDEX_REPAIR: &str = "
CREATE INDEX IF NOT EXISTS worker_discovery_age ON worker_discovery_index(age, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_movement ON worker_discovery_index(movement DESC, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_physicality ON worker_discovery_index(physicality DESC, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_ringcraft ON worker_discovery_index(ringcraft DESC, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_psychology ON worker_discovery_index(psychology DESC, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_fundamentals ON worker_discovery_index(fundamentals DESC, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_entertainment ON worker_discovery_index(entertainment DESC, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_condition ON worker_discovery_index(injury_days, fatigue, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_nationality ON worker_discovery_index(nationality COLLATE NOCASE, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_school ON worker_discovery_index(school COLLATE NOCASE, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_archetype ON worker_discovery_index(archetype COLLATE NOCASE, worker_id);
CREATE INDEX IF NOT EXISTS worker_discovery_discipline ON worker_discovery_index(primary_discipline COLLATE NOCASE, worker_id);
";

fn ensure_search_indexes(connection: &Connection) -> Result<(), PersistenceError> {
    let indexes_complete = connection
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='index' AND name='worker_discovery_discipline'",
            [],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    if !indexes_complete {
        connection.execute_batch(DISCOVERY_INDEX_REPAIR)?;
    }
    Ok(())
}

#[derive(Debug)]
struct DiscoveryHitRecord {
    id: String,
    match_reason: Option<String>,
    shortlist_ids: Vec<i32>,
    blacklisted: bool,
}

pub(super) fn migrate(
    connection: &mut Connection,
    record_upgrade: bool,
) -> Result<(), PersistenceError> {
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version == 7 {
        return Ok(());
    }
    if version != 6 {
        return Err(PersistenceError::InvalidDatabase);
    }
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute_batch(include_str!("../migrations/007_worker_discovery.sql"))?;
    let ids = tx
        .prepare("SELECT id FROM workers ORDER BY id")?
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for id in ids {
        let worker = super::career::worker(&tx, &id)?;
        refresh_worker(&tx, &worker)?;
    }
    let overview = read_overview(&tx)?;
    tx.execute(
        "UPDATE metadata SET value=?1 WHERE key='engine_version'",
        [CURRENT_ENGINE_VERSION],
    )?;
    tx.execute(
        "UPDATE metadata SET value='7' WHERE key='schema_version'",
        [],
    )?;
    tx.pragma_update(None, "user_version", 7)?;
    if record_upgrade {
        tx.execute(
            "INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('save_upgraded',?1,'{\"schemaVersion\":7}')",
            [&overview.current_date],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub(super) fn refresh_worker_if_present(
    connection: &Connection,
    worker: &Worker,
) -> Result<(), PersistenceError> {
    let exists = connection
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='worker_discovery_index'",
            [],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    if exists {
        refresh_worker(connection, worker)?;
    }
    Ok(())
}

fn refresh_worker(connection: &Connection, worker: &Worker) -> Result<(), PersistenceError> {
    let summary = worker.wrestling_style.summary(&worker.attributes);
    let languages = worker
        .identity
        .languages
        .iter()
        .map(|language| language.name.clone())
        .chain(std::iter::once(worker.language.clone()))
        .filter(|language| !language.trim().is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let has_aliases = connection
        .query_row(
            "SELECT 1 FROM pragma_table_info('worker_discovery_index') WHERE name='aliases'",
            [],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    let aliases = if has_aliases {
        connection.prepare("SELECT a.alias FROM character_aliases a JOIN characters c ON c.id=a.character_id WHERE c.worker_id=?1 AND a.knowledge!='private' ORDER BY a.started_on DESC")?.query_map([&worker.id],|r|r.get::<_,String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?
    } else {
        Vec::new()
    };
    let sql = if has_aliases {
        "INSERT INTO worker_discovery_index(worker_id,name,age,nationality,school,languages,biography,archetype,primary_discipline,overall,movement,physicality,ringcraft,psychology,fundamentals,entertainment,fatigue,morale,momentum,injury_days,aliases) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21) ON CONFLICT(worker_id) DO UPDATE SET name=excluded.name,age=excluded.age,nationality=excluded.nationality,school=excluded.school,languages=excluded.languages,biography=excluded.biography,archetype=excluded.archetype,primary_discipline=excluded.primary_discipline,overall=excluded.overall,movement=excluded.movement,physicality=excluded.physicality,ringcraft=excluded.ringcraft,psychology=excluded.psychology,fundamentals=excluded.fundamentals,entertainment=excluded.entertainment,fatigue=excluded.fatigue,morale=excluded.morale,momentum=excluded.momentum,injury_days=excluded.injury_days,aliases=excluded.aliases"
    } else {
        "INSERT INTO worker_discovery_index(worker_id,name,age,nationality,school,languages,biography,archetype,primary_discipline,overall,movement,physicality,ringcraft,psychology,fundamentals,entertainment,fatigue,morale,momentum,injury_days) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20) ON CONFLICT(worker_id) DO UPDATE SET name=excluded.name,age=excluded.age,nationality=excluded.nationality,school=excluded.school,languages=excluded.languages,biography=excluded.biography,archetype=excluded.archetype,primary_discipline=excluded.primary_discipline,overall=excluded.overall,movement=excluded.movement,physicality=excluded.physicality,ringcraft=excluded.ringcraft,psychology=excluded.psychology,fundamentals=excluded.fundamentals,entertainment=excluded.entertainment,fatigue=excluded.fatigue,morale=excluded.morale,momentum=excluded.momentum,injury_days=excluded.injury_days"
    };
    let mut values = vec![
        Value::Text(worker.id.clone()),
        Value::Text(worker.name.clone()),
        Value::Integer(i64::from(worker.age)),
        Value::Text(worker.nationality.clone()),
        Value::Text(worker.school.clone()),
        Value::Text(
            serde_json::to_string(&languages).map_err(|_| PersistenceError::InvalidDatabase)?,
        ),
        Value::Text(worker.background.clone()),
        Value::Text(summary.archetype),
        Value::Text(summary.primary.label().into()),
        Value::Integer(i64::from(summary.overall.get())),
        Value::Integer(i64::from(summary.groups.movement.get())),
        Value::Integer(i64::from(summary.groups.physicality.get())),
        Value::Integer(i64::from(summary.groups.ringcraft.get())),
        Value::Integer(i64::from(summary.groups.psychology.get())),
        Value::Integer(i64::from(summary.groups.fundamentals.get())),
        Value::Integer(i64::from(summary.groups.entertainment.get())),
        Value::Integer(i64::from(worker.condition.fatigue)),
        Value::Integer(i64::from(worker.condition.morale)),
        Value::Integer(i64::from(worker.condition.momentum)),
        Value::Integer(i64::from(worker.condition.injury_days)),
    ];
    if has_aliases {
        values.push(Value::Text(
            serde_json::to_string(&aliases).map_err(|_| PersistenceError::InvalidDatabase)?,
        ));
    }
    connection.execute(sql, params_from_iter(values))?;
    Ok(())
}

fn worker_match_score(
    name: &str,
    nationality: &str,
    school: &str,
    languages: &str,
    biography: &str,
    aliases: &str,
    query: &str,
) -> i64 {
    if query.is_empty() {
        return 0;
    }
    let query = query.to_lowercase();
    if serde_json::from_str::<Vec<String>>(aliases)
        .unwrap_or_default()
        .iter()
        .any(|alias| alias.to_lowercase().contains(&query))
    {
        return 95;
    }
    for (score, value) in [(100, name), (80, nationality), (70, school)] {
        if value.to_lowercase().contains(&query) {
            return score;
        }
    }
    if serde_json::from_str::<Vec<String>>(languages)
        .unwrap_or_default()
        .iter()
        .any(|language| language.to_lowercase().contains(&query))
    {
        return 60;
    }
    let limit = if query.chars().count() < 5 { 1 } else { 2 };
    if name
        .split_whitespace()
        .any(|part| levenshtein(&part.to_lowercase(), &query) <= limit)
    {
        return 50;
    }
    if biography.to_lowercase().contains(&query) {
        return 45;
    }
    0
}

fn levenshtein(left: &str, right: &str) -> usize {
    let mut previous = (0..=right.chars().count()).collect::<Vec<_>>();
    for (left_index, left_char) in left.chars().enumerate() {
        let mut current = vec![left_index + 1];
        for (right_index, right_char) in right.chars().enumerate() {
            current.push(
                (previous[right_index + 1] + 1)
                    .min(current[right_index] + 1)
                    .min(previous[right_index] + usize::from(left_char != right_char)),
            );
        }
        previous = current;
    }
    previous.last().copied().unwrap_or_default()
}

fn register_search_function(connection: &Connection) -> Result<(), PersistenceError> {
    connection.create_scalar_function(
        "wm_worker_match_score",
        7,
        FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
        |context| {
            Ok(worker_match_score(
                context.get::<String>(0)?.as_str(),
                context.get::<String>(1)?.as_str(),
                context.get::<String>(2)?.as_str(),
                context.get::<String>(3)?.as_str(),
                context.get::<String>(4)?.as_str(),
                context.get::<String>(5)?.as_str(),
                context.get::<String>(6)?.trim(),
            ))
        },
    )?;
    Ok(())
}

fn add_range(
    clauses: &mut Vec<String>,
    values: &mut Vec<Value>,
    column: &str,
    range: &NumberRange,
) {
    if let Some(minimum) = range.minimum {
        clauses.push(format!("d.{column} >= ?"));
        values.push(Value::Integer(i64::from(minimum)));
    }
    if let Some(maximum) = range.maximum {
        clauses.push(format!("d.{column} <= ?"));
        values.push(Value::Integer(i64::from(maximum)));
    }
}

fn placeholders(values: &[String], parameters: &mut Vec<Value>) -> String {
    parameters.extend(
        values
            .iter()
            .map(|value| Value::Text(value.trim().to_owned())),
    );
    std::iter::repeat_n("?", values.len())
        .collect::<Vec<_>>()
        .join(",")
}

fn add_choice_filter(
    clauses: &mut Vec<String>,
    parameters: &mut Vec<Value>,
    column: &str,
    included: &[String],
    excluded: &[String],
) {
    if !included.is_empty() {
        let selected = placeholders(included, parameters);
        clauses.push(format!("d.{column} COLLATE NOCASE IN ({selected})"));
    }
    if !excluded.is_empty() {
        let selected = placeholders(excluded, parameters);
        clauses.push(format!("d.{column} COLLATE NOCASE NOT IN ({selected})"));
    }
}

fn add_language_filter(
    clauses: &mut Vec<String>,
    parameters: &mut Vec<Value>,
    included: &[String],
    excluded: &[String],
) {
    if !included.is_empty() {
        let selected = placeholders(included, parameters);
        clauses.push(format!(
            "EXISTS (SELECT 1 FROM json_each(d.languages) language WHERE lower(CAST(language.value AS TEXT)) IN ({selected}))"
        ));
    }
    if !excluded.is_empty() {
        let selected = placeholders(excluded, parameters);
        clauses.push(format!(
            "NOT EXISTS (SELECT 1 FROM json_each(d.languages) language WHERE lower(CAST(language.value AS TEXT)) IN ({selected}))"
        ));
    }
}

fn search_query(request: &WorkerSearchRequest, count_only: bool) -> (String, Vec<Value>) {
    let query = request.text.trim();
    let mut parameters = Vec::<Value>::new();
    let candidates = if query.is_empty() {
        "SELECT d.*, 0 AS relevance_score FROM worker_discovery_index d".to_owned()
    } else {
        parameters.push(Value::Text(query.to_owned()));
        "SELECT d.*, wm_worker_match_score(d.name,d.nationality,d.school,d.languages,d.biography,d.aliases,?) AS relevance_score FROM worker_discovery_index d".to_owned()
    };
    let filters = &request.filters;
    let mut clauses = Vec::<String>::new();
    if !query.is_empty() {
        clauses.push("d.relevance_score > 0".into());
    }
    for (column, range) in [
        ("age", &filters.age),
        ("overall", &filters.overall),
        ("movement", &filters.movement),
        ("physicality", &filters.physicality),
        ("ringcraft", &filters.ringcraft),
        ("psychology", &filters.psychology),
        ("fundamentals", &filters.fundamentals),
        ("entertainment", &filters.entertainment),
    ] {
        add_range(&mut clauses, &mut parameters, column, range);
    }
    add_choice_filter(
        &mut clauses,
        &mut parameters,
        "nationality",
        &filters.nationalities,
        &filters.excluded_nationalities,
    );
    add_language_filter(
        &mut clauses,
        &mut parameters,
        &filters.languages,
        &filters.excluded_languages,
    );
    add_choice_filter(
        &mut clauses,
        &mut parameters,
        "school",
        &filters.schools,
        &filters.excluded_schools,
    );
    add_choice_filter(
        &mut clauses,
        &mut parameters,
        "archetype",
        &filters.archetypes,
        &filters.excluded_archetypes,
    );
    add_choice_filter(
        &mut clauses,
        &mut parameters,
        "primary_discipline",
        &filters.primary_disciplines,
        &filters.excluded_primary_disciplines,
    );
    match filters.availability {
        WorkerAvailability::Any => {}
        WorkerAvailability::Cleared => clauses.push("d.injury_days = 0".into()),
        WorkerAvailability::Injured => clauses.push("d.injury_days > 0".into()),
    }
    if let Some(shortlist_id) = filters.shortlist_id {
        clauses.push("EXISTS (SELECT 1 FROM worker_shortlist_members member WHERE member.worker_id=d.worker_id AND member.shortlist_id=?)".into());
        parameters.push(Value::Integer(i64::from(shortlist_id)));
    }
    match filters.blacklist {
        BlacklistMode::Include => {}
        BlacklistMode::Exclude => clauses.push(
            "NOT EXISTS (SELECT 1 FROM worker_blacklist blocked WHERE blocked.worker_id=d.worker_id)".into(),
        ),
        BlacklistMode::Only => clauses.push(
            "EXISTS (SELECT 1 FROM worker_blacklist blocked WHERE blocked.worker_id=d.worker_id)".into(),
        ),
    }
    let where_clause = if clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", clauses.join(" AND "))
    };
    if count_only {
        return (
            format!(
                "WITH candidates AS ({candidates}) SELECT COUNT(*) FROM candidates d{where_clause}"
            ),
            parameters,
        );
    }
    let mut order = request
        .sort
        .iter()
        .map(|sort| {
            let field = match sort.key {
                WorkerSortKey::Relevance => "d.relevance_score",
                WorkerSortKey::Name => "d.name COLLATE NOCASE",
                WorkerSortKey::Age => "d.age",
                WorkerSortKey::Overall => "d.overall",
                WorkerSortKey::Movement => "d.movement",
                WorkerSortKey::Physicality => "d.physicality",
                WorkerSortKey::Ringcraft => "d.ringcraft",
                WorkerSortKey::Psychology => "d.psychology",
                WorkerSortKey::Fundamentals => "d.fundamentals",
                WorkerSortKey::Entertainment => "d.entertainment",
                WorkerSortKey::Fatigue => "d.fatigue",
                WorkerSortKey::Morale => "d.morale",
                WorkerSortKey::Momentum => "d.momentum",
            };
            let direction = match sort.direction {
                SortDirection::Ascending => "ASC",
                SortDirection::Descending => "DESC",
            };
            format!("{field} {direction}")
        })
        .collect::<Vec<_>>();
    order.push("d.worker_id ASC".into());
    parameters.push(Value::Integer(request.limit as i64));
    parameters.push(Value::Integer(request.offset as i64));
    (
        format!(
            "WITH candidates AS ({candidates})
             SELECT d.worker_id,
                    CASE d.relevance_score WHEN 100 THEN 'Matched active ring name' WHEN 95 THEN 'Matched former ring name' WHEN 80 THEN 'Matched nationality' WHEN 70 THEN 'Matched training background' WHEN 60 THEN 'Matched language' WHEN 50 THEN 'Close ring-name match' WHEN 45 THEN 'Matched biography' END,
                    COALESCE((SELECT json_group_array(member.shortlist_id) FROM worker_shortlist_members member WHERE member.worker_id=d.worker_id),'[]'),
                    EXISTS (SELECT 1 FROM worker_blacklist blocked WHERE blocked.worker_id=d.worker_id)
             FROM candidates d{where_clause}
             ORDER BY {} LIMIT ? OFFSET ?",
            order.join(", ")
        ),
        parameters,
    )
}

fn valid_name(name: &str) -> bool {
    let length = name.trim().chars().count();
    (1..=48).contains(&length)
}

impl SaveRepository {
    pub fn worker_search(
        &self,
        save_id: &str,
        request: WorkerSearchRequest,
    ) -> Result<WorkerSearchPage, PersistenceError> {
        request
            .validate()
            .map_err(|message| PersistenceError::GameRule(message.into()))?;
        let connection = self.career_connection(save_id)?;
        ensure_search_indexes(&connection)?;
        register_search_function(&connection)?;
        let (count_sql, count_parameters) = search_query(&request, true);
        let total = connection.query_row(&count_sql, params_from_iter(count_parameters), |row| {
            row.get::<_, i64>(0)
        })? as usize;
        let (page_sql, page_parameters) = search_query(&request, false);
        let hits = connection
            .prepare(&page_sql)?
            .query_map(params_from_iter(page_parameters), |row| {
                let shortlist_ids: String = row.get(2)?;
                Ok(DiscoveryHitRecord {
                    id: row.get(0)?,
                    match_reason: row.get(1)?,
                    shortlist_ids: serde_json::from_str(&shortlist_ids).unwrap_or_default(),
                    blacklisted: row.get::<_, i64>(3)? != 0,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let company = read_overview(&connection)?.promotion_id;
        let rows = hits
            .into_iter()
            .map(|hit| {
                let worker = super::career::worker(&connection, &hit.id)?;
                let mut row = RosterRow::from(&worker);
                row.personality_description = worker
                    .identity
                    .for_viewer(Some(&company))
                    .describe(Some(&company))
                    .text;
                Ok(WorkerSearchHit {
                    worker: row,
                    match_reason: hit.match_reason,
                    shortlist_ids: hit.shortlist_ids,
                    blacklisted: hit.blacklisted,
                })
            })
            .collect::<Result<Vec<_>, PersistenceError>>()?;
        Ok(WorkerSearchPage { rows, total })
    }

    pub fn worker_filter_options(
        &self,
        save_id: &str,
    ) -> Result<WorkerFilterOptions, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        let distinct = |column: &str| -> Result<Vec<String>, PersistenceError> {
            let sql = format!(
                "SELECT DISTINCT {column} FROM worker_discovery_index WHERE trim({column}) <> '' ORDER BY {column} COLLATE NOCASE"
            );
            connection
                .prepare(&sql)?
                .query_map([], |row| row.get::<_, String>(0))?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(PersistenceError::from)
        };
        Ok(WorkerFilterOptions {
            nationalities: distinct("nationality")?,
            languages: connection
                .prepare(
                    "SELECT DISTINCT CAST(language.value AS TEXT) FROM worker_discovery_index d, json_each(d.languages) language WHERE trim(CAST(language.value AS TEXT)) <> '' ORDER BY CAST(language.value AS TEXT) COLLATE NOCASE",
                )?
                .query_map([], |row| row.get::<_, String>(0))?
                .collect::<rusqlite::Result<Vec<_>>>()?,
            schools: distinct("school")?,
            archetypes: distinct("archetype")?,
            primary_disciplines: distinct("primary_discipline")?,
        })
    }

    pub fn worker_discovery_lists(
        &self,
        save_id: &str,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        lists(&connection)
    }

    pub fn save_worker_view(
        &self,
        save_id: &str,
        id: Option<i32>,
        name: &str,
        request: WorkerSearchRequest,
        columns: Vec<String>,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        request
            .validate()
            .map_err(|message| PersistenceError::GameRule(message.into()))?;
        if !valid_name(name) {
            return Err(PersistenceError::GameRule(
                "Give the saved view a name of 1–48 characters.".into(),
            ));
        }
        let allowed = [
            "name",
            "age",
            "personality",
            "archetype",
            "overall",
            "movement",
            "physicality",
            "ringcraft",
            "psychology",
            "fundamentals",
            "entertainment",
            "fatigue",
            "morale",
            "momentum",
            "status",
        ];
        if columns.is_empty()
            || columns.len() > allowed.len()
            || columns
                .iter()
                .any(|column| !allowed.contains(&column.as_str()))
            || columns.iter().collect::<BTreeSet<_>>().len() != columns.len()
        {
            return Err(PersistenceError::GameRule(
                "The saved column layout is invalid.".into(),
            ));
        }
        let connection = self.career_connection(save_id)?;
        let encoded =
            serde_json::to_string(&serde_json::json!({ "request": request, "columns": columns }))
                .map_err(|_| PersistenceError::InvalidDatabase)?;
        match id {
            Some(id) => {
                connection.execute(
                    "UPDATE worker_saved_views SET name=?1,request=?2 WHERE id=?3",
                    params![name.trim(), encoded, id],
                )?;
            }
            None => {
                connection.execute(
                    "INSERT INTO worker_saved_views(name,request) VALUES(?1,?2)",
                    params![name.trim(), encoded],
                )?;
            }
        }
        lists(&connection)
    }

    pub fn delete_worker_view(
        &self,
        save_id: &str,
        id: i32,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        connection.execute("DELETE FROM worker_saved_views WHERE id=?1", [id])?;
        lists(&connection)
    }

    pub fn create_worker_shortlist(
        &self,
        save_id: &str,
        name: &str,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        if !valid_name(name) {
            return Err(PersistenceError::GameRule(
                "Give the shortlist a name of 1–48 characters.".into(),
            ));
        }
        let connection = self.career_connection(save_id)?;
        connection.execute(
            "INSERT INTO worker_shortlists(name) VALUES(?1)",
            [name.trim()],
        )?;
        lists(&connection)
    }

    pub fn delete_worker_shortlist(
        &self,
        save_id: &str,
        id: i32,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        connection.execute("DELETE FROM worker_shortlists WHERE id=?1", [id])?;
        lists(&connection)
    }

    pub fn set_worker_shortlist_member(
        &self,
        save_id: &str,
        shortlist_id: i32,
        worker_id: &str,
        included: bool,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        if included {
            connection.execute("INSERT OR IGNORE INTO worker_shortlist_members(shortlist_id,worker_id) VALUES(?1,?2)", params![shortlist_id, worker_id])?;
        } else {
            connection.execute(
                "DELETE FROM worker_shortlist_members WHERE shortlist_id=?1 AND worker_id=?2",
                params![shortlist_id, worker_id],
            )?;
        }
        lists(&connection)
    }

    pub fn set_worker_blacklisted(
        &self,
        save_id: &str,
        worker_id: &str,
        blacklisted: bool,
    ) -> Result<WorkerDiscoveryLists, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        if blacklisted {
            connection.execute(
                "INSERT OR IGNORE INTO worker_blacklist(worker_id) VALUES(?1)",
                [worker_id],
            )?;
        } else {
            connection.execute(
                "DELETE FROM worker_blacklist WHERE worker_id=?1",
                [worker_id],
            )?;
        }
        lists(&connection)
    }

    pub fn compare_workers(
        &self,
        save_id: &str,
        ids: &[String],
    ) -> Result<Vec<RosterRow>, PersistenceError> {
        if ids.is_empty() || ids.len() > 4 || ids.iter().collect::<BTreeSet<_>>().len() != ids.len()
        {
            return Err(PersistenceError::GameRule(
                "Choose between one and four different workers.".into(),
            ));
        }
        let connection = self.career_connection(save_id)?;
        let company = read_overview(&connection)?.promotion_id;
        ids.iter()
            .map(|id| {
                let worker = super::career::worker(&connection, id)?;
                let mut row = RosterRow::from(&worker);
                row.personality_description = worker
                    .identity
                    .for_viewer(Some(&company))
                    .describe(Some(&company))
                    .text;
                Ok(row)
            })
            .collect()
    }
}

fn lists(connection: &Connection) -> Result<WorkerDiscoveryLists, PersistenceError> {
    let saved_views = connection
        .prepare("SELECT id,name,request FROM worker_saved_views ORDER BY name COLLATE NOCASE,id")?
        .query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?
        .map(|row| {
            let (id, name, stored) = row?;
            let stored: serde_json::Value =
                serde_json::from_str(&stored).map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(SavedWorkerView {
                id,
                name,
                request: serde_json::from_value(stored["request"].clone())
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
                columns: serde_json::from_value(stored["columns"].clone())
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
            })
        })
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let shortlists = connection.prepare("SELECT s.id,s.name,COUNT(m.worker_id) FROM worker_shortlists s LEFT JOIN worker_shortlist_members m ON m.shortlist_id=s.id GROUP BY s.id,s.name ORDER BY s.name COLLATE NOCASE,s.id")?
        .query_map([], |row| Ok(WorkerShortlist { id: row.get(0)?, name: row.get(1)?, member_count: row.get::<_,i64>(2)? as usize }))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let blacklist_count =
        connection.query_row("SELECT COUNT(*) FROM worker_blacklist", [], |row| {
            row.get::<_, i64>(0)
        })? as usize;
    Ok(WorkerDiscoveryLists {
        saved_views,
        shortlists,
        blacklist_count,
    })
}
