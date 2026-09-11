use super::*;
use chrono::{Datelike, Days, NaiveDate};
use rusqlite::{TransactionBehavior, params};
use std::collections::{BTreeMap, BTreeSet};
use wm_domain::PromotionOverview;
use wm_domain::game::Worker;
use wm_domain::relationships::*;
use wm_sim::relationships::{
    InteractionContext, default_relationship, initial_management, initial_relationship,
    resolve_interaction,
};

fn encode<T: serde::Serialize>(value: &T) -> Result<String, PersistenceError> {
    serde_json::to_string(value).map_err(|_| PersistenceError::InvalidDatabase)
}

fn decode<T: serde::de::DeserializeOwned>(value: String) -> Result<T, PersistenceError> {
    serde_json::from_str(&value).map_err(|_| PersistenceError::InvalidDatabase)
}

pub(super) fn migrate(
    connection: &mut Connection,
    record_upgrade: bool,
) -> Result<(), PersistenceError> {
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let version: u32 = tx.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version == 6 {
        return Ok(());
    }
    if version != 5 {
        return Err(PersistenceError::InvalidDatabase);
    }
    tx.execute_batch(include_str!("../migrations/006_relationships.sql"))?;
    let promotion = read_overview(&tx)?;
    let ids = tx
        .prepare("SELECT id FROM workers ORDER BY id")?
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let workers = ids
        .iter()
        .map(|id| super::career::worker(&tx, id))
        .collect::<Result<Vec<_>, _>>()?;
    for subject in &workers {
        let management = initial_management(
            &promotion.promotion_id,
            &subject.id,
            &promotion.current_date,
        );
        management
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        tx.execute(
            "INSERT INTO management_relationships(company_id,worker_id,state) VALUES(?1,?2,?3)",
            params![promotion.promotion_id, subject.id, encode(&management)?],
        )?;
    }
    let mut schools = BTreeMap::<&str, Vec<&Worker>>::new();
    for worker in &workers {
        if !worker.school.trim().is_empty() {
            schools.entry(&worker.school).or_default().push(worker);
        }
    }
    for school in schools.values() {
        if school.len() < 2 {
            continue;
        }
        for (index, subject) in school.iter().enumerate() {
            let neighbour_indexes = BTreeSet::from([
                (index + 1) % school.len(),
                (index + school.len() - 1) % school.len(),
            ]);
            for neighbour_index in neighbour_indexes {
                let other = school[neighbour_index];
                let (state, memory) = relationship_at_date(
                    initial_relationship(subject, other, &promotion.founded_on),
                    &promotion,
                )?;
                state
                    .validate()
                    .map_err(|_| PersistenceError::InvalidDatabase)?;
                tx.execute(
                "INSERT INTO personal_relationships(subject_id,other_id,state) VALUES(?1,?2,?3)",
                params![subject.id, other.id, encode(&state)?],
            )?;
                if let Some(memory) = memory {
                    memory
                        .validate()
                        .map_err(|_| PersistenceError::InvalidDatabase)?;
                    tx.execute(
                    "INSERT INTO relationship_memories(id,subject_id,other_id,occurred_on,memory) VALUES(?1,?2,?3,?4,?5)",
                    params![memory.id, memory.subject_id, memory.other_id, memory.occurred_on, encode(&memory)?],
                )?;
                }
            }
        }
    }
    tx.execute(
        "UPDATE metadata SET value=?1 WHERE key='engine_version'",
        [CURRENT_ENGINE_VERSION],
    )?;
    tx.execute(
        "UPDATE metadata SET value='6' WHERE key='schema_version'",
        [],
    )?;
    tx.pragma_update(None, "user_version", 6)?;
    if record_upgrade {
        tx.execute(
            "INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('save_upgraded',?1,'{\"schemaVersion\":6}')",
            [&promotion.current_date],
        )?;
    }
    tx.commit()?;
    Ok(())
}

fn relationship_at_date(
    relationship: (PersonalRelationshipState, Option<RelationshipMemory>),
    promotion: &PromotionOverview,
) -> Result<(PersonalRelationshipState, Option<RelationshipMemory>), PersistenceError> {
    let (mut state, memory) = relationship;
    let mut cursor = NaiveDate::parse_from_str(&promotion.founded_on, "%Y-%m-%d")
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    let as_of = NaiveDate::parse_from_str(&promotion.current_date, "%Y-%m-%d")
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    if cursor > as_of {
        return Err(PersistenceError::InvalidDatabase);
    }
    while cursor < as_of && state.scores.tension > 0 {
        cursor = cursor
            .checked_add_days(Days::new(1))
            .ok_or(PersistenceError::InvalidDatabase)?;
        if cursor.day() == 1 || cursor.day() == 15 {
            state.scores.tension -= 1;
            state.revision = state.revision.saturating_add(1);
            state.updated_on = cursor.format("%Y-%m-%d").to_string();
        }
    }
    Ok((state, memory))
}

fn stored_personal(
    connection: &Connection,
    subject_id: &str,
    other_id: &str,
) -> Result<Option<PersonalRelationshipState>, PersistenceError> {
    let Some(json) = connection
        .query_row(
            "SELECT state FROM personal_relationships WHERE subject_id=?1 AND other_id=?2",
            params![subject_id, other_id],
            |row| row.get::<_, String>(0),
        )
        .optional()?
    else {
        return Ok(None);
    };
    let state: PersonalRelationshipState = decode(json)?;
    state
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    if state.subject_id != subject_id || state.other_id != other_id {
        return Err(PersistenceError::InvalidDatabase);
    }
    Ok(Some(state))
}

fn management(
    connection: &Connection,
    worker_id: &str,
    company_id: &str,
) -> Result<ManagementRelationshipState, PersistenceError> {
    let json: String = connection
        .query_row(
            "SELECT state FROM management_relationships WHERE company_id=?1 AND worker_id=?2",
            params![company_id, worker_id],
            |row| row.get(0),
        )
        .optional()?
        .ok_or(PersistenceError::InvalidDatabase)?;
    let state: ManagementRelationshipState = decode(json)?;
    state
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    if state.worker_id != worker_id || state.company_id != company_id {
        return Err(PersistenceError::InvalidDatabase);
    }
    Ok(state)
}

fn personal(
    connection: &Connection,
    subject_id: &str,
    other_id: &str,
) -> Result<PersonalRelationshipState, PersistenceError> {
    if let Some(state) = stored_personal(connection, subject_id, other_id)? {
        return Ok(state);
    }
    let subject = super::career::worker(connection, subject_id)?;
    let other = super::career::worker(connection, other_id)?;
    let promotion = read_overview(connection)?;
    relationship_at_date(
        (
            default_relationship(&subject.id, &other.id, &promotion.founded_on),
            None,
        ),
        &promotion,
    )
    .map(|value| value.0)
}

fn recent_work(
    connection: &Connection,
    worker_id: &str,
    as_of: &str,
) -> Result<Option<String>, PersistenceError> {
    let cutoff = NaiveDate::parse_from_str(as_of, "%Y-%m-%d")
        .map_err(|_| PersistenceError::InvalidDatabase)?
        .checked_sub_days(Days::new(28))
        .ok_or(PersistenceError::InvalidDatabase)?
        .format("%Y-%m-%d")
        .to_string();
    connection
        .query_row(
            "SELECT json_extract(h.report,'$.title') FROM worker_history h JOIN shows s ON s.id=h.show_id WHERE h.worker_id=?1 AND s.show_date>=?2 AND s.show_date<=?3 ORDER BY s.show_date DESC,h.id DESC LIMIT 1",
            params![worker_id, cutoff, as_of],
            |row| row.get(0),
        )
        .optional()
        .map_err(Into::into)
}

pub(super) fn profile(
    connection: &Connection,
    worker_id: &str,
    company_id: &str,
) -> Result<RelationshipProfile, PersistenceError> {
    let promotion = read_overview(connection)?;
    let as_of = promotion.current_date.clone();
    let worker = super::career::worker(connection, worker_id)?;
    let management = management(connection, worker_id, company_id)?;
    let rows = connection
        .prepare(
            "SELECT w.id,w.name,r.state FROM workers w LEFT JOIN personal_relationships r ON r.subject_id=?1 AND r.other_id=w.id WHERE w.id<>?1 ORDER BY w.id",
        )?
        .query_map([worker_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut states = Vec::with_capacity(rows.len());
    for (other_id, name, stored) in rows {
        let state = if let Some(json) = stored {
            let state: PersonalRelationshipState = decode(json)?;
            state
                .validate()
                .map_err(|_| PersistenceError::InvalidDatabase)?;
            if state.subject_id != worker_id || state.other_id != other_id {
                return Err(PersistenceError::InvalidDatabase);
            }
            state
        } else {
            relationship_at_date(
                (
                    default_relationship(&worker.id, &other_id, &promotion.founded_on),
                    None,
                ),
                &promotion,
            )?
            .0
        };
        states.push((state, name));
    }
    states.sort_by(|(left, left_name), (right, right_name)| {
        significance(right.scores)
            .cmp(&significance(left.scores))
            .then_with(|| left_name.cmp(right_name))
            .then_with(|| left.other_id.cmp(&right.other_id))
    });
    let has_targets = !states.is_empty();
    let mut views = vec![];
    for (state, name) in states.into_iter().take(12) {
        let memories = connection
            .prepare("SELECT memory,occurred_on FROM relationship_memories WHERE subject_id=?1 AND other_id=?2 ORDER BY occurred_on DESC,id DESC LIMIT 6")?
            .query_map(params![worker_id, state.other_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?
            .map(|row| {
                let (json, occurred_on) = row?;
                let memory: RelationshipMemory = decode(json)?;
                memory.validate().map_err(|_| PersistenceError::InvalidDatabase)?;
                if memory.subject_id != worker_id
                    || memory.other_id != state.other_id
                    || memory.occurred_on != occurred_on
                {
                    return Err(PersistenceError::InvalidDatabase);
                }
                Ok(RelationshipMemoryView {
                    id: memory.id,
                    occurred_on: memory.occurred_on,
                    kind: memory.kind,
                    summary: memory.summary,
                    active: memory.active_until.as_deref().is_none_or(|until| until >= as_of.as_str()),
                })
            })
            .collect::<Result<Vec<_>, PersistenceError>>()?;
        views.push(PersonalRelationshipView {
            other_id: state.other_id,
            other_name: name,
            summary: relationship_summary(state.scores),
            signals: signals(state.scores),
            memories,
        });
    }
    let history = connection
        .prepare("SELECT outcome FROM player_interactions WHERE company_id=?1 AND worker_id=?2 ORDER BY occurred_on DESC,rowid DESC LIMIT 20")?
        .query_map(params![company_id, worker_id], |row| row.get::<_, String>(0))?
        .map(|json| {
            let outcome: InteractionOutcome = decode(json?)?;
            outcome.validate().map_err(|_| PersistenceError::InvalidDatabase)?;
            if outcome.worker_id != worker_id {
                return Err(PersistenceError::InvalidDatabase);
            }
            Ok(outcome)
        })
        .collect::<Result<Vec<_>, PersistenceError>>()?;
    let used: u32 = connection.query_row(
        "SELECT COALESCE(SUM(attention_cost),0) FROM player_interactions WHERE company_id=?1 AND occurred_on=?2",
        params![company_id, as_of],
        |row| row.get(0),
    )?;
    let remaining = DAILY_INTERACTION_ATTENTION.saturating_sub(used);
    let recent = recent_work(connection, worker_id, &as_of)?;
    let options = InteractionKind::ALL
        .iter()
        .map(|kind| {
            option(
                connection,
                *kind,
                &worker,
                company_id,
                &management,
                &as_of,
                remaining,
                recent.is_some(),
                has_targets,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RelationshipProfile {
        rule_version: RELATIONSHIP_RULE_VERSION,
        management: ManagementRelationshipView {
            summary: management_summary(management.scores),
            signals: signals(management.scores),
            revision: management.revision,
        },
        personal: views,
        interaction_options: options,
        interaction_history: history,
        attention_remaining: remaining,
        attention_limit: DAILY_INTERACTION_ATTENTION,
    })
}

#[allow(clippy::too_many_arguments)]
fn option(
    connection: &Connection,
    kind: InteractionKind,
    worker: &wm_domain::game::Worker,
    company_id: &str,
    management: &ManagementRelationshipState,
    as_of: &str,
    remaining: u32,
    has_recent_work: bool,
    has_targets: bool,
) -> Result<InteractionOption, PersistenceError> {
    let prior: Option<String> = connection
        .query_row(
            "SELECT occurred_on FROM player_interactions WHERE company_id=?1 AND worker_id=?2 AND kind=?3 ORDER BY occurred_on DESC,rowid DESC LIMIT 1",
            params![company_id, worker.id, kind_key(kind)],
            |row| row.get(0),
        )
        .optional()?;
    let cooldown_until = match (prior.as_deref(), kind.cooldown_days()) {
        (Some(date), Some(days)) => Some(
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .map_err(|_| PersistenceError::InvalidDatabase)?
                .checked_add_days(Days::new(days))
                .ok_or(PersistenceError::InvalidDatabase)?
                .format("%Y-%m-%d")
                .to_string(),
        ),
        _ => None,
    };
    let unavailable_reason = if remaining < kind.attention_cost() {
        Some("No management attention remains today.".into())
    } else if kind == InteractionKind::IntroduceYourself && prior.is_some() {
        Some("You have already introduced yourself.".into())
    } else if kind == InteractionKind::PraiseRecentWork && !has_recent_work {
        Some("There is no performance from the last 28 days to discuss.".into())
    } else if kind == InteractionKind::OfferEncouragement
        && worker.condition.morale >= 60
        && worker.condition.confidence >= 60
    {
        Some("They do not currently need a morale or confidence intervention.".into())
    } else if kind == InteractionKind::ClearTheAir && management.scores.tension < 20 {
        Some("There is no meaningful tension with management to address.".into())
    } else if kind.requires_target() && !has_targets {
        Some("There is no colleague available to discuss.".into())
    } else if cooldown_until.as_deref().is_some_and(|until| until > as_of) {
        Some(format!(
            "This conversation will feel repetitive until {}.",
            cooldown_until.as_deref().unwrap_or_default()
        ))
    } else {
        None
    };
    let visible_cooldown = cooldown_until.filter(|until| until.as_str() > as_of);
    Ok(InteractionOption {
        kind,
        label: kind.label().into(),
        description: kind.description().into(),
        enabled: unavailable_reason.is_none(),
        unavailable_reason,
        attention_cost: kind.attention_cost(),
        cooldown_until: visible_cooldown,
        requires_target: kind.requires_target(),
    })
}

impl SaveRepository {
    pub fn interaction_targets(
        &self,
        save_id: &str,
        worker_id: &str,
        search: &str,
        offset: u32,
        limit: u32,
    ) -> Result<InteractionTargetPage, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        let subject = super::career::worker(&connection, worker_id)?;
        let promotion = read_overview(&connection)?;
        let search = search.chars().take(80).collect::<String>();
        let total = connection.query_row(
            "SELECT COUNT(*) FROM workers WHERE id<>?1 AND instr(lower(name),lower(?2))>0",
            params![worker_id, search],
            |row| row.get(0),
        )?;
        let workers = connection
            .prepare("SELECT id,name FROM workers WHERE id<>?1 AND instr(lower(name),lower(?2))>0 ORDER BY name COLLATE NOCASE,id LIMIT ?3 OFFSET ?4")?
            .query_map(params![worker_id, search, limit.clamp(1, 32), offset], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let rows = workers
            .into_iter()
            .map(|(other_id, name)| {
                let state = if let Some(state) = stored_personal(&connection, worker_id, &other_id)?
                {
                    state
                } else {
                    relationship_at_date(
                        (
                            default_relationship(&subject.id, &other_id, &promotion.founded_on),
                            None,
                        ),
                        &promotion,
                    )?
                    .0
                };
                Ok(InteractionTarget {
                    worker_id: other_id,
                    name,
                    relationship: relationship_summary(state.scores),
                })
            })
            .collect::<Result<Vec<_>, PersistenceError>>()?;
        Ok(InteractionTargetPage { rows, total })
    }

    pub fn interact_with_worker(
        &self,
        request: InteractionRequest,
    ) -> Result<InteractionOutcome, PersistenceError> {
        request.validate().map_err(PersistenceError::GameRule)?;
        let mut connection = self.career_connection(&request.save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        if let Some(json) = tx
            .query_row(
                "SELECT outcome FROM player_interactions WHERE request_id=?1",
                [&request.request_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?
        {
            let prior: InteractionOutcome = decode(json)?;
            prior
                .validate()
                .map_err(|_| PersistenceError::InvalidDatabase)?;
            if prior.worker_id == request.worker_id
                && prior.kind == request.kind
                && prior.context_worker_id == request.context_worker_id
            {
                return Ok(prior);
            }
            return Err(PersistenceError::GameRule(
                "That interaction request ID was already used for a different conversation.".into(),
            ));
        }
        let promotion = read_overview(&tx)?;
        let mut worker = super::career::worker(&tx, &request.worker_id)?;
        let relationship_profile = profile(&tx, &request.worker_id, &promotion.promotion_id)?;
        if relationship_profile.management.revision != request.expected_revision {
            return Err(PersistenceError::GameRule(
                "The relationship changed. Reload the profile before speaking.".into(),
            ));
        }
        let selected = relationship_profile
            .interaction_options
            .iter()
            .find(|option| option.kind == request.kind)
            .ok_or(PersistenceError::InvalidDatabase)?;
        if !selected.enabled {
            return Err(PersistenceError::GameRule(
                selected
                    .unavailable_reason
                    .clone()
                    .unwrap_or_else(|| "That conversation is not currently available.".into()),
            ));
        }
        if selected.requires_target != request.context_worker_id.is_some() {
            return Err(PersistenceError::GameRule(if selected.requires_target {
                "Choose a colleague to discuss.".into()
            } else {
                "That conversation does not use a colleague.".into()
            }));
        }
        let colleague = if let Some(other_id) = request.context_worker_id.as_deref() {
            let exists: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM workers WHERE id=?1)",
                [other_id],
                |row| row.get(0),
            )?;
            if !exists {
                return Err(PersistenceError::GameRule(
                    "Choose an available colleague.".into(),
                ));
            }
            let other = super::career::worker(&tx, other_id)?;
            let state = personal(&tx, &request.worker_id, other_id)?;
            Some((
                other.name,
                relationship_summary(state.scores),
                other_id.to_owned(),
            ))
        } else {
            None
        };
        let prior_count: u32 = tx.query_row(
            "SELECT COUNT(*) FROM player_interactions WHERE company_id=?1 AND worker_id=?2 AND kind=?3",
            params![promotion.promotion_id, request.worker_id, kind_key(request.kind)],
            |row| row.get(0),
        )?;
        let current = management(&tx, &request.worker_id, &promotion.promotion_id)?;
        let resolution = resolve_interaction(
            &worker,
            current,
            request.kind,
            InteractionContext {
                request_id: request.request_id.clone(),
                occurred_on: promotion.current_date.clone(),
                recent_work: recent_work(&tx, &request.worker_id, &promotion.current_date)?,
                colleague,
                prior_count,
            },
        );
        resolution
            .management
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        resolution
            .outcome
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        worker.condition.morale =
            (worker.condition.morale + resolution.outcome.morale_delta).clamp(0, 100);
        worker.condition.confidence =
            (worker.condition.confidence + resolution.outcome.confidence_delta).clamp(0, 100);
        super::career::write_worker(&tx, &worker)?;
        tx.execute(
            "UPDATE management_relationships SET state=?1 WHERE company_id=?2 AND worker_id=?3",
            params![
                encode(&resolution.management)?,
                promotion.promotion_id,
                request.worker_id
            ],
        )?;
        tx.execute(
            "INSERT INTO player_interactions(request_id,company_id,worker_id,kind,occurred_on,context_worker_id,attention_cost,management_revision,outcome) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![request.request_id,promotion.promotion_id,request.worker_id,kind_key(request.kind),promotion.current_date,request.context_worker_id,request.kind.attention_cost(),resolution.management.revision,encode(&resolution.outcome)?],
        )?;
        tx.execute(
            "INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('worker_interaction',?1,?2)",
            params![promotion.current_date, encode(&resolution.outcome)?],
        )?;
        super::news::publish_interaction(&tx, &worker.name, &resolution.outcome)?;
        tx.commit()?;
        Ok(resolution.outcome)
    }

    /// Internal producer boundary for future match, team and world systems.
    pub fn record_personal_relationship_event(
        &self,
        save_id: &str,
        event: PersonalRelationshipEvent,
        expected_revision: u32,
    ) -> Result<PersonalRelationshipState, PersistenceError> {
        event.validate().map_err(PersistenceError::GameRule)?;
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        if let Some(json) = tx
            .query_row(
                "SELECT memory FROM relationship_memories WHERE id=?1",
                [&event.memory.id],
                |row| row.get::<_, String>(0),
            )
            .optional()?
        {
            let prior: RelationshipMemory = decode(json)?;
            if prior == event.memory {
                return personal(&tx, &prior.subject_id, &prior.other_id);
            }
            return Err(PersistenceError::GameRule(
                "Conflicting relationship memory ID.".into(),
            ));
        }
        let current_date = read_overview(&tx)?.current_date;
        if event.memory.occurred_on != current_date {
            return Err(PersistenceError::GameRule(
                "Relationship events must use the current game date.".into(),
            ));
        }
        let mut state = personal(&tx, &event.memory.subject_id, &event.memory.other_id)?;
        if state.revision != expected_revision {
            return Err(PersistenceError::GameRule(
                "That relationship changed. Reload before recording another event.".into(),
            ));
        }
        let impact = event.memory.impact;
        state.scores = state.scores.adjusted(
            impact.affinity,
            impact.respect,
            impact.trust,
            impact.tension,
        );
        state.revision = state.revision.saturating_add(1);
        state.updated_on = event.memory.occurred_on.clone();
        state
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        tx.execute(
            "INSERT INTO personal_relationships(subject_id,other_id,state) VALUES(?1,?2,?3) ON CONFLICT(subject_id,other_id) DO UPDATE SET state=excluded.state",
            params![state.subject_id, state.other_id, encode(&state)?],
        )?;
        tx.execute(
            "INSERT INTO relationship_memories(id,subject_id,other_id,occurred_on,memory) VALUES(?1,?2,?3,?4,?5)",
            params![event.memory.id,event.memory.subject_id,event.memory.other_id,event.memory.occurred_on,encode(&event.memory)?],
        )?;
        tx.commit()?;
        Ok(state)
    }
}

pub(super) fn decay(connection: &Connection, as_of: &str) -> Result<(), PersistenceError> {
    if !as_of.ends_with("-01") && !as_of.ends_with("-15") {
        return Ok(());
    }
    let rows = connection
        .prepare("SELECT subject_id,other_id,state FROM personal_relationships WHERE json_extract(state,'$.scores.tension')>0")?
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for (subject, other, json) in rows {
        let mut state: PersonalRelationshipState = decode(json)?;
        state
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        state.scores.tension -= 1;
        state.revision = state.revision.saturating_add(1);
        state.updated_on = as_of.into();
        connection.execute(
            "UPDATE personal_relationships SET state=?1 WHERE subject_id=?2 AND other_id=?3",
            params![encode(&state)?, subject, other],
        )?;
    }
    let rows = connection
        .prepare("SELECT company_id,worker_id,state FROM management_relationships WHERE json_extract(state,'$.scores.tension')>0")?
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for (company, worker, json) in rows {
        let mut state: ManagementRelationshipState = decode(json)?;
        state
            .validate()
            .map_err(|_| PersistenceError::InvalidDatabase)?;
        state.scores.tension -= 1;
        state.revision = state.revision.saturating_add(1);
        state.updated_on = as_of.into();
        connection.execute(
            "UPDATE management_relationships SET state=?1 WHERE company_id=?2 AND worker_id=?3",
            params![encode(&state)?, company, worker],
        )?;
    }
    Ok(())
}

fn significance(scores: RelationshipScores) -> u32 {
    scores.tension as u32 * 2
        + scores.affinity.unsigned_abs()
        + scores.respect.unsigned_abs()
        + scores.trust.unsigned_abs()
}

fn kind_key(kind: InteractionKind) -> &'static str {
    match kind {
        InteractionKind::IntroduceYourself => "introduceYourself",
        InteractionKind::CheckIn => "checkIn",
        InteractionKind::PraiseRecentWork => "praiseRecentWork",
        InteractionKind::OfferEncouragement => "offerEncouragement",
        InteractionKind::AskForCreativeInput => "askForCreativeInput",
        InteractionKind::DiscussColleague => "discussColleague",
        InteractionKind::ClearTheAir => "clearTheAir",
    }
}
