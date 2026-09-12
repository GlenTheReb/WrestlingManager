use super::*;
use chrono::NaiveDate;
use rusqlite::{TransactionBehavior, params};
use wm_domain::characters::*;
use wm_domain::identity::{PersonalityField, QualityField};
use wm_domain::relationships::ManagementRelationshipState;

fn encode<T: serde::Serialize>(value: &T) -> Result<String, PersistenceError> {
    serde_json::to_string(value).map_err(|_| PersistenceError::InvalidDatabase)
}
fn decode<T: serde::de::DeserializeOwned>(value: String) -> Result<T, PersistenceError> {
    serde_json::from_str(&value).map_err(|_| PersistenceError::InvalidDatabase)
}
fn parse_alignment(value: &str) -> Result<AlignmentIntent, PersistenceError> {
    match value {
        "face" => Ok(AlignmentIntent::Face),
        "heel" => Ok(AlignmentIntent::Heel),
        "tweener" => Ok(AlignmentIntent::Tweener),
        "unaligned" => Ok(AlignmentIntent::Unaligned),
        _ => Err(PersistenceError::InvalidDatabase),
    }
}
fn alignment(value: AlignmentIntent) -> &'static str {
    match value {
        AlignmentIntent::Face => "face",
        AlignmentIntent::Heel => "heel",
        AlignmentIntent::Tweener => "tweener",
        AlignmentIntent::Unaligned => "unaligned",
    }
}
fn status(value: &str) -> Result<CharacterStatus, PersistenceError> {
    match value {
        "planned" => Ok(CharacterStatus::Planned),
        "active" => Ok(CharacterStatus::Active),
        "retired" => Ok(CharacterStatus::Retired),
        _ => Err(PersistenceError::InvalidDatabase),
    }
}
fn knowledge(value: &str) -> Result<IdentityKnowledge, PersistenceError> {
    match value {
        "private" => Ok(IdentityKnowledge::Private),
        "rumoured" => Ok(IdentityKnowledge::Rumoured),
        "public" => Ok(IdentityKnowledge::Public),
        _ => Err(PersistenceError::InvalidDatabase),
    }
}
fn change_status(value: &str) -> Result<ChangeStatus, PersistenceError> {
    match value {
        "proposed" => Ok(ChangeStatus::Proposed),
        "negotiating" => Ok(ChangeStatus::Negotiating),
        "accepted" => Ok(ChangeStatus::Accepted),
        "refused" => Ok(ChangeStatus::Refused),
        "ready" => Ok(ChangeStatus::Ready),
        "launched" => Ok(ChangeStatus::Launched),
        "cancelled" => Ok(ChangeStatus::Cancelled),
        _ => Err(PersistenceError::InvalidDatabase),
    }
}

fn assessment(
    worker: &wm_domain::game::Worker,
    quality: Option<QualityField>,
    personality: Option<PersonalityField>,
) -> i32 {
    quality
        .and_then(|field| worker.identity.qualities.get(&field))
        .or_else(|| personality.and_then(|field| worker.identity.personality.get(&field)))
        .and_then(|value| value.value)
        .map_or(50, |value| value.get())
}

fn validate_request_id(value: &str) -> Result<(), PersistenceError> {
    if value.trim().is_empty() || value.len() > 100 || value.chars().any(char::is_control) {
        return Err(super::career::rule(
            "Character requests need a valid request ID.",
        ));
    }
    Ok(())
}

fn action_was_applied(
    connection: &Connection,
    request: &CharacterActionRequest,
    action: &str,
) -> Result<bool, PersistenceError> {
    let receipt = connection
        .query_row(
            "SELECT worker_id,action FROM character_action_receipts WHERE request_id=?1",
            [&request.request_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()?;
    match receipt {
        None => Ok(false),
        Some((worker_id, recorded_action))
            if worker_id == request.worker_id && recorded_action == action =>
        {
            Ok(true)
        }
        Some(_) => Err(super::career::rule(
            "That request ID was already used for another character action.",
        )),
    }
}

pub(super) fn migrate(
    connection: &mut Connection,
    record_upgrade: bool,
) -> Result<(), PersistenceError> {
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version == 8 {
        return Ok(());
    }
    if version != 7 {
        return Err(PersistenceError::InvalidDatabase);
    }
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute_batch(include_str!("../migrations/008_characters.sql"))?;
    let date = read_overview(&tx)?.current_date;
    let company = read_overview(&tx)?.promotion_id;
    let workers = tx
        .prepare("SELECT id,name FROM workers ORDER BY id")?
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for (worker_id, name) in workers {
        let character_id = format!("character-{worker_id}");
        tx.execute(
            "INSERT INTO persons(worker_id,legal_name) VALUES(?1,?2)",
            params![worker_id, name],
        )?;
        tx.execute("INSERT INTO characters(id,worker_id,ring_name,alignment_intent,status,masked,concealed,gimmick,created_on) VALUES(?1,?2,?3,'unaligned','active',0,0,?4,?5)", params![character_id,worker_id,name,encode(&GimmickBrief::default())?,date])?;
        tx.execute("INSERT INTO character_tenures(character_id,company_id,brand,started_on,ended_on,knowledge) VALUES(?1,?2,NULL,?3,NULL,'public')", params![character_id,company,date])?;
        tx.execute("INSERT INTO character_aliases(character_id,alias,started_on,ended_on,knowledge) VALUES(?1,?2,?3,NULL,'public')", params![character_id,name,date])?;
    }
    let ids = tx
        .prepare("SELECT id FROM workers ORDER BY id")?
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for id in ids {
        let worker = super::career::worker(&tx, &id)?;
        super::discovery::refresh_worker_if_present(&tx, &worker)?;
    }
    tx.execute(
        "UPDATE metadata SET value=?1 WHERE key='engine_version'",
        [CURRENT_ENGINE_VERSION],
    )?;
    tx.execute(
        "UPDATE metadata SET value='8' WHERE key='schema_version'",
        [],
    )?;
    tx.pragma_update(None, "user_version", 8)?;
    if record_upgrade {
        tx.execute("INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('save_upgraded',?1,'{\"schemaVersion\":8}')", [&date])?;
    }
    tx.commit()?;
    Ok(())
}

fn read_character(
    connection: &Connection,
    worker_id: &str,
    current_only: bool,
) -> Result<Vec<CharacterIdentity>, PersistenceError> {
    let current_clause = if current_only {
        "AND t.ended_on IS NULL"
    } else {
        ""
    };
    let sql = format!(
        "SELECT c.id,c.revision,c.ring_name,c.alignment_intent,c.status,c.masked,c.concealed,c.gimmick,t.company_id,t.brand,t.started_on,t.ended_on,t.knowledge FROM characters c JOIN character_tenures t ON t.character_id=c.id WHERE c.worker_id=?1 {current_clause} ORDER BY t.started_on DESC,c.id"
    );
    let mut statement = connection.prepare(&sql)?;
    let rows = statement
        .query_map([worker_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, bool>(5)?,
                row.get::<_, bool>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, Option<String>>(11)?,
                row.get::<_, String>(12)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    rows.into_iter().map(|r| {
        let aliases = connection.prepare("SELECT alias FROM character_aliases WHERE character_id=?1 AND knowledge!='private' ORDER BY started_on DESC,id DESC")?
            .query_map([&r.0], |row| row.get::<_,String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(CharacterIdentity { id:r.0, revision:r.1, ring_name:r.2, alignment_intent:parse_alignment(&r.3)?, status:status(&r.4)?, masked:r.5, concealed:r.6, gimmick:decode(r.7)?, company_id:r.8, brand:r.9, started_on:r.10, ended_on:r.11, aliases, identity_knowledge:knowledge(&r.12)? })
    }).collect()
}

fn read_change(
    connection: &Connection,
    worker_id: &str,
) -> Result<Option<CharacterChange>, PersistenceError> {
    connection.query_row("SELECT id,request_id,proposed_ring_name,proposed_alignment,masked,concealed,proposed_gimmick,status,revision,proposed_on,intended_launch_on,launched_on,worker_response,readiness,risk,advice FROM character_changes WHERE worker_id=?1 AND status NOT IN ('launched','cancelled') ORDER BY id DESC LIMIT 1", [worker_id], |r| Ok((r.get::<_,i64>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?,r.get::<_,String>(3)?,r.get::<_,bool>(4)?,r.get::<_,bool>(5)?,r.get::<_,String>(6)?,r.get::<_,String>(7)?,r.get::<_,i32>(8)?,r.get::<_,String>(9)?,r.get::<_,Option<String>>(10)?,r.get::<_,Option<String>>(11)?,r.get::<_,String>(12)?,r.get::<_,String>(13)?,r.get::<_,String>(14)?,r.get::<_,String>(15)?)))
        .optional()?.map(|r| Ok(CharacterChange { id:r.0, request_id:r.1, proposed_ring_name:r.2, proposed_alignment:parse_alignment(&r.3)?, proposed_masked:r.4, proposed_concealed:r.5, proposed_gimmick:decode(r.6)?, status:change_status(&r.7)?, revision:r.8, proposed_on:r.9, intended_launch_on:r.10, launched_on:r.11, worker_response:r.12, readiness:r.13, risk:r.14, advice:decode(r.15)? })).transpose()
}

pub(super) fn profile(
    connection: &Connection,
    worker_id: &str,
) -> Result<CharacterProfile, PersistenceError> {
    let legal_name = connection
        .query_row(
            "SELECT legal_name FROM persons WHERE worker_id=?1",
            [worker_id],
            |r| r.get(0),
        )
        .optional()?
        .flatten();
    let company = read_overview(connection)?.promotion_id;
    let mut identities = read_character(connection, worker_id, false)?;
    let active_index = identities
        .iter()
        .position(|identity| identity.ended_on.is_none() && identity.company_id == company)
        .or_else(|| {
            identities
                .iter()
                .position(|identity| identity.ended_on.is_none())
        })
        .ok_or(PersistenceError::InvalidDatabase)?;
    let active = identities.remove(active_index);
    let history = identities;
    let responses = connection.prepare("SELECT occurred_on,perceived_role,response,intensity,acceptance,intent_match,context FROM audience_response_evidence WHERE character_id=?1 ORDER BY occurred_on DESC LIMIT 30")?
        .query_map([&active.id], |r| Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?,r.get::<_,String>(3)?,r.get::<_,String>(4)?,r.get::<_,String>(5)?,r.get::<_,String>(6)?)))?.collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter().map(|(date,role,response,intensity,acceptance,intent_match,context)| Ok(AudienceResponseEvidence {
            date,
            perceived_role: match role.as_str(){"face"=>PerceivedRole::Face,"heel"=>PerceivedRole::Heel,"mixed"=>PerceivedRole::Mixed,"unclear"=>PerceivedRole::Unclear,_=>return Err(PersistenceError::InvalidDatabase)},
            response: match response.as_str(){"cheered"=>AudienceResponse::Cheered,"booed"=>AudienceResponse::Booed,"mixed"=>AudienceResponse::Mixed,"indifferent"=>AudienceResponse::Indifferent,_=>return Err(PersistenceError::InvalidDatabase)},
            intensity: match intensity.as_str(){"mild"=>ReactionIntensity::Mild,"moderate"=>ReactionIntensity::Moderate,"strong"=>ReactionIntensity::Strong,_=>return Err(PersistenceError::InvalidDatabase)},
            acceptance: match acceptance.as_str(){"embraced"=>CharacterAcceptance::Embraced,"accepted"=>CharacterAcceptance::Accepted,"uncertain"=>CharacterAcceptance::Uncertain,"rejected"=>CharacterAcceptance::Rejected,_=>return Err(PersistenceError::InvalidDatabase)},
            intent_match: match intent_match.as_str(){"matched"=>IntentMatch::Matched,"mismatched"=>IntentMatch::Mismatched,"ambiguous"=>IntentMatch::Ambiguous,_=>return Err(PersistenceError::InvalidDatabase)},
            context
        })).collect::<Result<_,_>>()?;
    Ok(CharacterProfile {
        legal_name,
        active,
        history,
        audience_responses: responses,
        pending_change: read_change(connection, worker_id)?,
    })
}

pub(super) fn active_ring_name(
    connection: &Connection,
    worker_id: &str,
) -> Result<Option<String>, PersistenceError> {
    if connection
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='characters'",
            [],
            |_| Ok(()),
        )
        .optional()?
        .is_none()
    {
        return Ok(None);
    }
    connection.query_row("SELECT c.ring_name FROM characters c JOIN character_tenures t ON t.character_id=c.id WHERE c.worker_id=?1 AND c.status='active' AND t.company_id=(SELECT id FROM promotions ORDER BY id LIMIT 1) AND t.ended_on IS NULL ORDER BY t.started_on DESC LIMIT 1", [worker_id], |r| r.get(0)).optional().map_err(Into::into)
}

impl SaveRepository {
    pub fn propose_character_change(
        &self,
        request: ProposeCharacterChangeRequest,
    ) -> Result<CharacterProfile, PersistenceError> {
        let mut connection = self.career_connection(&request.save_id)?;
        let ring_name = validate_public_name(&request.ring_name).map_err(super::career::rule)?;
        validate_gimmick(&request.gimmick).map_err(super::career::rule)?;
        validate_request_id(&request.request_id)?;
        if request
            .intended_launch_on
            .as_deref()
            .is_some_and(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err())
        {
            return Err(super::career::rule("Choose a valid public launch date."));
        }
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        if let Some(existing) = tx
            .query_row(
                "SELECT worker_id FROM character_changes WHERE request_id=?1",
                [&request.request_id],
                |r| r.get::<_, String>(0),
            )
            .optional()?
        {
            if existing != request.worker_id {
                return Err(super::career::rule("That request ID was already used."));
            }
            tx.commit()?;
            return profile(&connection, &request.worker_id);
        }
        if request.expected_revision
            != read_change(&tx, &request.worker_id)?
                .map(|c| c.revision)
                .unwrap_or(0)
        {
            return Err(super::career::rule(
                "The character plan changed. Refresh before saving.",
            ));
        }
        tx.execute(
            "UPDATE character_changes SET status='cancelled',revision=revision+1 WHERE worker_id=?1 AND status NOT IN ('launched','cancelled')",
            [&request.worker_id],
        )?;
        let worker = super::career::worker(&tx, &request.worker_id)?;
        let overview = read_overview(&tx)?;
        if request
            .intended_launch_on
            .as_deref()
            .is_some_and(|date| date < overview.current_date.as_str())
        {
            return Err(super::career::rule(
                "The public launch date cannot be in the past.",
            ));
        }
        let current_character = profile(&tx, &request.worker_id)?.active;
        let duplicate: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM characters c JOIN character_tenures t ON t.character_id=c.id WHERE lower(c.ring_name)=lower(?1) AND t.company_id=?2 AND t.ended_on IS NULL AND c.worker_id<>?3)", params![ring_name,overview.promotion_id,request.worker_id], |r|r.get(0))?;
        if duplicate {
            return Err(super::career::rule(
                "That ring name is already active in this company.",
            ));
        }
        if request.concealed && !request.masked {
            return Err(super::career::rule(
                "A concealed identity needs a mask or equivalent full concealment.",
            ));
        }
        let management: ManagementRelationshipState = tx
            .query_row(
                "SELECT state FROM management_relationships WHERE company_id=?1 AND worker_id=?2",
                params![overview.promotion_id, request.worker_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?
            .map(decode)
            .transpose()?
            .unwrap_or_else(|| {
                wm_sim::relationships::initial_management(
                    &overview.promotion_id,
                    &request.worker_id,
                    &overview.current_date,
                )
            });
        let adaptability = assessment(&worker, Some(QualityField::PersonalAdaptability), None);
        let creativity = assessment(&worker, Some(QualityField::Creativity), None);
        let professionalism = assessment(&worker, Some(QualityField::Professionalism), None);
        let ego = assessment(&worker, None, Some(PersonalityField::Ego));
        let style_fit = request
            .gimmick
            .tags
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(&worker.style))
            || request
                .gimmick
                .core_fantasy
                .to_lowercase()
                .contains(&worker.style.to_lowercase());
        let continuity = i32::from(ring_name.eq_ignore_ascii_case(&current_character.ring_name))
            * 8
            + i32::from(request.alignment_intent == current_character.alignment_intent) * 5;
        let acceptance_score = ((worker.condition.morale
            + worker.condition.confidence
            + adaptability
            + creativity
            + professionalism
            + (50 + management.scores.trust / 2)
            - ego / 2)
            / 6
            + i32::from(style_fit) * 8
            + continuity
            - i32::from(request.masked != current_character.masked) * 5)
            .clamp(0, 100);
        let (state, response) = if acceptance_score < 15 {
            ("refused", "I don't believe this direction is right for me.")
        } else if acceptance_score < 27 {
            (
                "negotiating",
                "I can work with this, but I want to discuss the presentation.",
            )
        } else {
            ("accepted", "I'm comfortable committing to this direction.")
        };
        let before_debut = worker.condition.matches == 0;
        let risk = if before_debut {
            "Low: the character has not debuted, so the audience has no established identity to unlearn."
        } else if request
            .intended_launch_on
            .as_deref()
            .is_some_and(|d| d > overview.current_date.as_str())
        {
            "Moderate: preparation helps, but an established identity still needs a clear story reason."
        } else {
            "High: an abrupt change to an exposed active identity may confuse the audience and unsettle the performer."
        };
        let readiness = if before_debut {
            "Ready for debut"
        } else if request
            .intended_launch_on
            .as_deref()
            .is_some_and(|date| date > overview.current_date.as_str())
        {
            "Preparation scheduled"
        } else if request.intended_launch_on.is_some() {
            "Ready to launch"
        } else {
            "Launch date required"
        };
        let mut advice = vec![
            "This changes presentation and audience interpretation, never permanent wrestling ratings."
                .to_string(),
            format!(
                "Worker response considers morale, confidence, management trust, ego, adaptability, professionalism, creativity and concept continuity (rule set 1; assessment {acceptance_score}/100)."
            ),
        ];
        if style_fit {
            advice.push("The concept connects clearly to the worker's established style.".into());
        }
        if !before_debut {
            advice
                .push("Use a storyline beat or credible absence to explain the transition.".into());
        }
        if request.gimmick.tags.is_empty() {
            advice.push("Add a few clear wrestling presentation tags so agents can give useful contextual advice.".into());
        }
        tx.execute("INSERT INTO character_changes(worker_id,request_id,proposed_ring_name,proposed_alignment,proposed_gimmick,masked,concealed,status,revision,proposed_on,intended_launch_on,launched_on,worker_response,readiness,risk,advice) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,1,?9,?10,NULL,?11,?12,?13,?14)", params![request.worker_id,request.request_id,ring_name,alignment(request.alignment_intent),encode(&request.gimmick)?,request.masked,request.concealed,state,overview.current_date,request.intended_launch_on,response,readiness,risk,encode(&advice)?])?;
        tx.commit()?;
        profile(&connection, &request.worker_id)
    }

    pub fn launch_character_change(
        &self,
        request: CharacterActionRequest,
    ) -> Result<CharacterProfile, PersistenceError> {
        let mut connection = self.career_connection(&request.save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        validate_request_id(&request.request_id)?;
        if action_was_applied(&tx, &request, "launch")? {
            tx.commit()?;
            return profile(&connection, &request.worker_id);
        }
        let id = request
            .change_id
            .ok_or_else(|| super::career::rule("Choose a character plan to launch."))?;
        let change = read_change(&tx, &request.worker_id)?
            .filter(|c| c.id == id)
            .ok_or_else(|| super::career::rule("That character plan is no longer available."))?;
        if change.revision != request.expected_revision {
            return Err(super::career::rule(
                "The character plan changed. Refresh before launching.",
            ));
        }
        if matches!(
            change.status,
            ChangeStatus::Refused | ChangeStatus::Negotiating
        ) {
            return Err(super::career::rule(
                "Resolve the worker's response before launching this character.",
            ));
        }
        let overview = read_overview(&tx)?;
        let date = overview.current_date;
        if change
            .intended_launch_on
            .as_deref()
            .is_some_and(|d| d > date.as_str())
        {
            return Err(super::career::rule(
                "The planned public launch date has not arrived.",
            ));
        }
        let old = profile(&tx, &request.worker_id)?.active;
        let ended = tx.execute(
            "UPDATE character_tenures SET ended_on=?1 WHERE character_id=?2 AND company_id=?3 AND ended_on IS NULL",
            params![date, old.id, overview.promotion_id],
        )?;
        if ended == 0 {
            return Err(super::career::rule(
                "The active character is no longer assigned to this company.",
            ));
        }
        let remains_active: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM character_tenures WHERE character_id=?1 AND ended_on IS NULL)",
            [&old.id],
            |row| row.get(0),
        )?;
        if !remains_active {
            tx.execute(
                "UPDATE characters SET status='retired',revision=revision+1 WHERE id=?1",
                [&old.id],
            )?;
            tx.execute(
                "UPDATE character_aliases SET ended_on=?1 WHERE character_id=?2 AND ended_on IS NULL",
                params![date, old.id],
            )?;
        }
        let new_id = format!("character-{}-{}", request.worker_id, id);
        tx.execute("INSERT INTO characters(id,worker_id,ring_name,alignment_intent,status,masked,concealed,gimmick,created_on) SELECT ?1,worker_id,proposed_ring_name,proposed_alignment,'active',masked,concealed,proposed_gimmick,?2 FROM character_changes WHERE id=?3", params![new_id,date,id])?;
        tx.execute("INSERT INTO character_tenures(character_id,company_id,brand,started_on,ended_on,knowledge) VALUES(?1,?2,NULL,?3,NULL,'public')",params![new_id,overview.promotion_id,date])?;
        tx.execute("INSERT INTO character_aliases(character_id,alias,started_on,knowledge) VALUES(?1,?2,?3,'public')",params![new_id,change.proposed_ring_name,date])?;
        tx.execute("UPDATE character_changes SET status='launched',revision=revision+1,launched_on=?1 WHERE id=?2",params![date,id])?;
        tx.execute(
            "UPDATE workers SET name=?1 WHERE id=?2",
            params![change.proposed_ring_name, request.worker_id],
        )?;
        tx.execute("INSERT INTO character_action_receipts(request_id,worker_id,action,occurred_on) VALUES(?1,?2,'launch',?3)",params![request.request_id,request.worker_id,date])?;
        tx.execute("INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('character_launched',?1,json_object('workerId',?2,'characterId',?3,'ringName',?4))",params![date,request.worker_id,new_id,change.proposed_ring_name])?;
        let worker = super::career::worker(&tx, &request.worker_id)?;
        super::discovery::refresh_worker_if_present(&tx, &worker)?;
        tx.commit()?;
        profile(&connection, &request.worker_id)
    }

    pub fn set_character_retired(
        &self,
        request: CharacterActionRequest,
        retired: bool,
    ) -> Result<CharacterProfile, PersistenceError> {
        let mut connection = self.career_connection(&request.save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        validate_request_id(&request.request_id)?;
        let requested_action = if retired { "retire" } else { "revive" };
        if action_was_applied(&tx, &request, requested_action)? {
            tx.commit()?;
            return profile(&connection, &request.worker_id);
        }
        let active = profile(&tx, &request.worker_id)?.active;
        if active.revision != request.expected_revision {
            return Err(super::career::rule(
                "The character changed. Refresh before updating its status.",
            ));
        }
        if (active.status == CharacterStatus::Retired) == retired {
            return Err(super::career::rule(if retired {
                "That character is already retired."
            } else {
                "That character is already active."
            }));
        }
        let date = read_overview(&tx)?.current_date;
        tx.execute(
            "UPDATE characters SET status=?1,revision=revision+1 WHERE id=?2",
            params![if retired { "retired" } else { "active" }, active.id],
        )?;
        tx.execute("INSERT INTO character_action_receipts(request_id,worker_id,action,occurred_on) VALUES(?1,?2,?3,?4)",params![request.request_id,request.worker_id,if retired{"retire"}else{"revive"},date])?;
        tx.execute("INSERT INTO domain_events(event_type,occurred_on,payload) VALUES(?1,?2,json_object('workerId',?3,'characterId',?4))",params![if retired{"character_retired"}else{"character_revived"},date,request.worker_id,active.id])?;
        tx.commit()?;
        profile(&connection, &request.worker_id)
    }
}
