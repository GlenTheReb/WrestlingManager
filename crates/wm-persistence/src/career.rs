use super::*;
use chrono::{Days, NaiveDate};
use rusqlite::{TransactionBehavior, params};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::BTreeSet;
use wm_domain::game::*;
use wm_sim::{consequences::media_for, content, planning, runtime::Session};

const MIGRATION_V2: &str = include_str!("../migrations/002_gameplay.sql");
pub(super) fn rule(message: impl Into<String>) -> PersistenceError {
    PersistenceError::GameRule(message.into())
}
fn encode<T: Serialize>(value: &T) -> Result<String, PersistenceError> {
    serde_json::to_string(value).map_err(|_| PersistenceError::InvalidDatabase)
}
fn decode<T: DeserializeOwned>(value: String) -> Result<T, PersistenceError> {
    serde_json::from_str(&value).map_err(|_| PersistenceError::InvalidDatabase)
}
fn next_date(date: &str, days: u64) -> Result<String, PersistenceError> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .ok()
        .and_then(|d| d.checked_add_days(Days::new(days)))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .ok_or(PersistenceError::InvalidDatabase)
}

pub(super) fn initialise_gameplay(
    connection: &mut Connection,
    migration: bool,
) -> Result<(), PersistenceError> {
    let version: u32 = connection.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if version >= 2 {
        super::news::migrate(connection)?;
        super::ratings::migrate(connection, migration)?;
        super::identity::migrate(connection, migration)?;
        super::relationships::migrate(connection, migration)?;
        super::discovery::migrate(connection, migration)?;
        return super::characters::migrate(connection, migration);
    }
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    if transaction.pragma_query_value::<u32, _>(None, "user_version", |r| r.get(0))? >= 2 {
        transaction.commit()?;
        super::news::migrate(connection)?;
        super::ratings::migrate(connection, migration)?;
        super::identity::migrate(connection, migration)?;
        super::relationships::migrate(connection, migration)?;
        super::discovery::migrate(connection, migration)?;
        return super::characters::migrate(connection, migration);
    }
    let seed = metadata_value(&transaction, "seed")?.ok_or(PersistenceError::InvalidDatabase)?;
    let seed = wm_domain::Seed::parse(&seed)?.get();
    let date: String = transaction.query_row(
        "SELECT promotions.current_date FROM promotions LIMIT 1",
        [],
        |r| r.get(0),
    )?;
    transaction.execute_batch(MIGRATION_V2)?;
    let pack = content::base_pack();
    for worker in content::generate_world(seed, &pack) {
        write_worker(&transaction, &worker)?;
    }
    for agent in content::initial_agents() {
        transaction.execute(
            "INSERT INTO agents(id,data) VALUES(?1,?2)",
            params![agent.id, encode(&agent)?],
        )?;
    }
    transaction.execute("INSERT INTO shows(name,show_date,status,revision) VALUES('UWF Thursday Night',?1,'draft',0)",[&date])?;
    transaction.execute("INSERT INTO audience(id,trust) VALUES(1,65)", [])?;
    // This transaction is a recoverable schema-2 checkpoint. Later migrations may fail and retry.
    transaction.execute(
        "UPDATE metadata SET value='0.2.0' WHERE key='engine_version'",
        [],
    )?;
    transaction.execute(
        "UPDATE metadata SET value='2' WHERE key='schema_version'",
        [],
    )?;
    transaction.pragma_update(None, "user_version", 2)?;
    if migration {
        transaction.execute("INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('save_upgraded',?1,'{\"schemaVersion\":2}')",[&date])?;
    }
    transaction.commit()?;
    super::news::migrate(connection)?;
    super::ratings::migrate(connection, migration)?;
    super::identity::migrate(connection, migration)?;
    super::relationships::migrate(connection, migration)?;
    super::discovery::migrate(connection, migration)?;
    super::characters::migrate(connection, migration)
}

pub(super) fn write_worker(connection: &Connection, w: &Worker) -> Result<(), PersistenceError> {
    w.identity
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    w.wrestling_style
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    connection.execute("INSERT INTO workers(id,name,age,style,nationality,language,school,background,personality,ambition,weight_kg,appearance_fee,attributes,wrestling_style,condition,moves,identity)
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
        ON CONFLICT(id) DO UPDATE SET age=excluded.age,style=excluded.style,attributes=excluded.attributes,wrestling_style=excluded.wrestling_style,condition=excluded.condition,moves=excluded.moves",
        params![w.id,w.name,w.age,w.style,w.nationality,w.language,w.school,w.background,w.personality,w.ambition,w.weight_kg,w.appearance_fee,
            encode(&w.attributes)?,encode(&w.wrestling_style)?,encode(&w.condition)?,encode(&w.moves)?,encode(&w.identity)?])?;
    super::discovery::refresh_worker_if_present(connection, w)?;
    Ok(())
}

pub(super) fn worker(connection: &Connection, id: &str) -> Result<Worker, PersistenceError> {
    let row=connection.query_row("SELECT name,age,style,nationality,language,school,background,personality,ambition,weight_kg,appearance_fee,attributes,wrestling_style,condition,moves FROM workers WHERE id=?1",[id],|r|{
        Ok((r.get::<_,String>(0)?,r.get::<_,i32>(1)?,r.get::<_,String>(2)?,r.get::<_,String>(3)?,r.get::<_,String>(4)?,r.get::<_,String>(5)?,r.get::<_,String>(6)?,r.get::<_,String>(7)?,r.get::<_,String>(8)?,r.get::<_,i32>(9)?,r.get::<_,i32>(10)?,r.get::<_,String>(11)?,r.get::<_,String>(12)?,r.get::<_,String>(13)?,r.get::<_,String>(14)?))
    }).optional()?.ok_or_else(||rule("That wrestler is not in this career."))?;
    let identity: wm_domain::identity::PersonIdentity = decode(connection.query_row(
        "SELECT identity FROM workers WHERE id=?1",
        [id],
        |r| r.get(0),
    )?)?;
    identity
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    let worker = Worker {
        id: id.into(),
        name: super::characters::active_ring_name(connection, id)?.unwrap_or(row.0),
        age: row.1,
        style: row.2,
        nationality: row.3,
        language: row.4,
        school: row.5,
        background: row.6,
        personality: row.7,
        ambition: row.8,
        identity,
        weight_kg: row.9,
        appearance_fee: row.10,
        attributes: decode(row.11)?,
        wrestling_style: decode(row.12)?,
        condition: decode(row.13)?,
        moves: decode(row.14)?,
    };
    worker
        .wrestling_style
        .validate()
        .map_err(|_| PersistenceError::InvalidDatabase)?;
    Ok(worker)
}
fn agents(connection: &Connection) -> Result<Vec<RoadAgent>, PersistenceError> {
    connection
        .prepare("SELECT data FROM agents ORDER BY id")?
        .query_map([], |r| r.get::<_, String>(0))?
        .map(|s| decode(s?))
        .collect()
}
fn card(connection: &Connection, id: i32) -> Result<ShowCard, PersistenceError> {
    let mut show = connection
        .query_row(
            "SELECT name,show_date,status,revision FROM shows WHERE id=?1",
            [id],
            |r| {
                Ok(ShowCard {
                    id,
                    name: r.get(0)?,
                    date: r.get(1)?,
                    status: r.get(2)?,
                    revision: r.get(3)?,
                    segments: Vec::new(),
                    capacity_seconds: 7200,
                })
            },
        )
        .optional()?
        .ok_or_else(|| rule("That show is unavailable."))?;
    show.segments = connection
        .prepare("SELECT id,title,plan FROM segments WHERE show_id=?1 ORDER BY position,id")?
        .query_map([id], |r| {
            Ok((
                r.get::<_, i32>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })?
        .map(|row| {
            let (id, title, content) = row?;
            Ok(Segment {
                id,
                title,
                content: decode(content)?,
            })
        })
        .collect::<Result<_, PersistenceError>>()?;
    Ok(show)
}

fn profile_participants(
    connection: &Connection,
    plan: &SegmentPlan,
    report: Option<&SegmentReport>,
) -> Result<Vec<ProfileParticipant>, PersistenceError> {
    let ids = match plan {
        SegmentPlan::Match(plan) => vec![plan.worker_a.as_str(), plan.worker_b.as_str()],
        SegmentPlan::Angle(plan) => plan.participants.iter().map(String::as_str).collect(),
    };
    ids.into_iter()
        .map(|worker_id| {
            let recorded_name = report.and_then(|report| {
                report
                    .changes
                    .iter()
                    .find(|change| change.worker_id == worker_id)
                    .map(|change| change.name.clone())
            });
            let name = match recorded_name {
                Some(name) => name,
                None => worker(connection, worker_id)?.name,
            };
            Ok(ProfileParticipant {
                worker_id: worker_id.to_owned(),
                name,
            })
        })
        .collect()
}

fn profile_history(
    connection: &Connection,
    worker_id: &str,
) -> Result<Vec<ProfileAppearance>, PersistenceError> {
    let rows = connection
        .prepare(
            "SELECT h.show_id,sh.name,sh.show_date,h.segment_id,seg.plan,h.report
             FROM worker_history h
             JOIN shows sh ON sh.id=h.show_id
             JOIN segments seg ON seg.id=h.segment_id
             WHERE h.worker_id=?1
             ORDER BY h.id DESC
             LIMIT 30",
        )?
        .query_map([worker_id], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    rows.into_iter()
        .map(|(show_id, show_name, date, segment_id, plan, report)| {
            let plan: SegmentPlan = decode(plan)?;
            let report: SegmentReport = decode(report)?;
            let kind = match &plan {
                SegmentPlan::Match(_) => ProfileAppearanceKind::Match,
                SegmentPlan::Angle(_) => ProfileAppearanceKind::Angle,
            };
            Ok(ProfileAppearance {
                show_id,
                show_name,
                date,
                segment_id,
                kind,
                title: report.title.clone(),
                participants: profile_participants(connection, &plan, Some(&report))?,
                winner_id: report.winner_id.clone(),
                result: report.result.clone(),
                duration_seconds: report.duration_seconds,
                performance: report.performance,
                reasons: report.reasons,
            })
        })
        .collect()
}

fn next_profile_booking(
    connection: &Connection,
    worker_id: &str,
) -> Result<Option<ProfileBooking>, PersistenceError> {
    let row = connection
        .query_row(
            "SELECT sh.id,sh.name,sh.show_date,seg.id,seg.title,seg.plan
             FROM shows sh
             JOIN segments seg ON seg.show_id=sh.id
             WHERE sh.status!='complete' AND (
               (json_extract(seg.plan,'$.kind')='match' AND
                (json_extract(seg.plan,'$.plan.workerA')=?1 OR json_extract(seg.plan,'$.plan.workerB')=?1))
               OR
               (json_extract(seg.plan,'$.kind')='angle' AND EXISTS(
                 SELECT 1 FROM json_each(seg.plan,'$.plan.participants') WHERE value=?1
               ))
             )
             AND NOT EXISTS(
               SELECT 1 FROM worker_history h
               WHERE h.worker_id=?1 AND h.segment_id=seg.id
             )
             ORDER BY sh.show_date,seg.position,seg.id
             LIMIT 1",
            [worker_id],
            |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                ))
            },
        )
        .optional()?;
    row.map(|(show_id, show_name, date, segment_id, title, plan)| {
        let plan: SegmentPlan = decode(plan)?;
        let kind = match &plan {
            SegmentPlan::Match(_) => ProfileAppearanceKind::Match,
            SegmentPlan::Angle(_) => ProfileAppearanceKind::Angle,
        };
        Ok(ProfileBooking {
            show_id,
            show_name,
            date,
            segment_id,
            kind,
            title,
            participants: profile_participants(connection, &plan, None)?,
        })
    })
    .transpose()
}

fn profile_news(
    connection: &Connection,
    worker_id: &str,
) -> Result<Vec<NewsItem>, PersistenceError> {
    connection
        .prepare(
            "SELECT id,category,title,body,occurred_on,show_id,worker_id,is_read
             FROM news_items INDEXED BY news_date WHERE worker_id=?1
             ORDER BY occurred_on DESC,id DESC LIMIT 8",
        )?
        .query_map([worker_id], |row| {
            Ok(NewsItem {
                id: row.get(0)?,
                category: row.get(1)?,
                title: row.get(2)?,
                body: row.get(3)?,
                date: row.get(4)?,
                show_id: row.get(5)?,
                worker_id: row.get(6)?,
                read: row.get(7)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

fn get_session(connection: &Connection, id: i32) -> Result<Session, PersistenceError> {
    let data = connection
        .query_row(
            "SELECT snapshot FROM show_runtime WHERE show_id=?1",
            [id],
            |r| r.get::<_, String>(0),
        )
        .optional()?
        .ok_or_else(|| rule("Start the show before opening its live feed."))?;
    decode(data)
}
fn get_report(connection: &Connection, id: i32) -> Result<Option<ShowReport>, PersistenceError> {
    connection
        .query_row("SELECT report FROM shows WHERE id=?1", [id], |r| {
            r.get::<_, Option<String>>(0)
        })
        .optional()?
        .flatten()
        .map(decode)
        .transpose()
}
fn live_view(connection: &Connection, id: i32) -> Result<LiveView, PersistenceError> {
    let session = get_session(connection, id)?;
    let mut events = connection
        .prepare(
            "SELECT data FROM simulation_events WHERE show_id=?1 ORDER BY sequence DESC LIMIT 80",
        )?
        .query_map([id], |r| r.get::<_, String>(0))?
        .map(|v| decode(v?))
        .collect::<Result<Vec<SimEvent>, PersistenceError>>()?;
    events.reverse();
    Ok(session.view(events, get_report(connection, id)?))
}

impl SaveRepository {
    pub(super) fn upgrade_career(&self, id: &SaveId) -> Result<(), PersistenceError> {
        let path = self.path_for(id);
        let source = self.open_existing(id)?;
        validate_compatibility(&source)?;
        let version: u32 = source.pragma_query_value(None, "user_version", |r| r.get(0))?;
        let backup = path.with_extension(format!("sqlite3.v{version}.bak"));
        if !backup.exists() {
            let temporary = Builder::new()
                .prefix(".backup-")
                .tempfile_in(&self.saves_directory)
                .map_err(PersistenceError::Filesystem)?
                .into_temp_path();
            source.backup("main", &temporary, None)?;
            let check = open_read_only(&temporary)?;
            let integrity: String = check.query_row("PRAGMA integrity_check", [], |r| r.get(0))?;
            if integrity != "ok" {
                return Err(PersistenceError::InvalidDatabase);
            }
            drop(check);
            temporary
                .persist_noclobber(&backup)
                .map_err(|e| PersistenceError::Filesystem(e.error))?;
        }
        // A pre-existing file is not evidence of a successful backup.
        let verified = open_read_only(&backup)?;
        validate_compatibility(&verified)?;
        let integrity: String = verified.query_row("PRAGMA integrity_check", [], |r| r.get(0))?;
        if integrity != "ok" || read_overview(&verified)? != read_overview(&source)? {
            return Err(rule(
                "The existing upgrade backup does not match this career. Preserve both files before resolving the backup conflict.",
            ));
        }
        drop(verified);
        drop(source);
        let mut connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        initialise_gameplay(&mut connection, true)
    }

    pub(super) fn career_connection(&self, save_id: &str) -> Result<Connection, PersistenceError> {
        self.load_game(save_id)?;
        let connection = Connection::open_with_flags(
            self.path_for(&SaveId::parse(save_id)?),
            OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        connection.pragma_update(None, "foreign_keys", true)?;
        Ok(connection)
    }

    pub fn career_office(&self, save_id: &str) -> Result<CareerOffice, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        let id = connection.query_row(
            "SELECT id FROM shows WHERE status!='complete' ORDER BY id LIMIT 1",
            [],
            |r| r.get(0),
        )?;
        let ids = connection
            .prepare("SELECT id FROM shows WHERE status='complete' ORDER BY id DESC LIMIT 8")?
            .query_map([], |r| r.get::<_, i32>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let media = connection
            .prepare("SELECT id,author,text,posted_on FROM media_posts ORDER BY id DESC LIMIT 30")?
            .query_map([], |r| {
                Ok(MediaPost {
                    id: r.get(0)?,
                    author: r.get(1)?,
                    text: r.get(2)?,
                    date: r.get(3)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(CareerOffice {
            promotion: read_overview(&connection)?,
            show: card(&connection, id)?,
            agents: agents(&connection)?,
            roster_count: connection.query_row("SELECT COUNT(*) FROM workers", [], |r| r.get(0))?,
            media,
            recent_shows: ids
                .into_iter()
                .map(|id| card(&connection, id))
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn roster_page(
        &self,
        save_id: &str,
        search: &str,
        offset: u32,
        limit: u32,
    ) -> Result<RosterPage, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        let search = search.chars().take(80).collect::<String>();
        let total = connection.query_row(
            "SELECT COUNT(*) FROM workers WHERE instr(lower(name),lower(?1))>0 OR instr(lower(nationality),lower(?1))>0 OR instr(lower(school),lower(?1))>0 OR EXISTS(SELECT 1 FROM json_each(workers.identity,'$.languages') WHERE instr(lower(json_extract(value,'$.name')),lower(?1))>0)",
            [&search],
            |r| r.get(0),
        )?;
        let ids = connection
            .prepare("SELECT id FROM workers WHERE instr(lower(name),lower(?1))>0 OR instr(lower(nationality),lower(?1))>0 OR instr(lower(school),lower(?1))>0 OR EXISTS(SELECT 1 FROM json_each(workers.identity,'$.languages') WHERE instr(lower(json_extract(value,'$.name')),lower(?1))>0) ORDER BY name COLLATE NOCASE,id LIMIT ?2 OFFSET ?3")?
            .query_map(params![search,limit.clamp(1,64),offset],|row|row.get::<_,String>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let company = read_overview(&connection)?.promotion_id;
        let rows = ids
            .iter()
            .map(|id| {
                worker(&connection, id).map(|worker| {
                    let mut row = RosterRow::from(&worker);
                    row.personality_description = worker
                        .identity
                        .for_viewer(Some(&company))
                        .describe(Some(&company))
                        .text;
                    row
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RosterPage { rows, total })
    }

    pub fn worker_profile(
        &self,
        save_id: &str,
        id: &str,
    ) -> Result<WorkerProfile, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        let history = profile_history(&connection, id)?;
        let next_booking = next_profile_booking(&connection, id)?;
        let recent_news = profile_news(&connection, id)?;
        let mut worker = worker(&connection, id)?;
        let promotion = read_overview(&connection)?;
        worker.identity = worker.identity.for_viewer(Some(&promotion.promotion_id));
        let personality_description = worker.identity.describe(Some(&promotion.promotion_id));
        worker.personality = personality_description.text.clone();
        worker.ambition = worker.identity.motivation_text();
        let biography = worker.identity.biography_text(
            &worker.name,
            &worker.nationality,
            &worker.school,
            &worker.background,
            worker.condition.matches,
        );
        let exceptional_traits =
            super::identity::ledger(&connection, id)?.view(Some(&promotion.promotion_id));
        let relationships =
            super::relationships::profile(&connection, id, &promotion.promotion_id)?;
        let character = super::characters::profile(&connection, id)?;
        let wrestling = worker.wrestling_style.summary(&worker.attributes);
        Ok(WorkerProfile {
            worker,
            wrestling,
            company: ProfileCompany {
                id: promotion.promotion_id,
                name: promotion.name,
                initials: promotion.initials,
                region: promotion.region,
            },
            history,
            next_booking,
            recent_news,
            personality_description,
            biography,
            exceptional_traits,
            relationships,
            character,
        })
    }

    pub fn show_card(&self, save_id: &str, id: i32) -> Result<ShowCard, PersistenceError> {
        card(&self.career_connection(save_id)?, id)
    }

    pub fn agent_advice(
        &self,
        save_id: &str,
        plan: MatchPlan,
    ) -> Result<AgentAdvice, PersistenceError> {
        let connection = self.career_connection(save_id)?;
        let agent = agents(&connection)?
            .into_iter()
            .find(|a| a.id == plan.agent_id)
            .ok_or_else(|| rule("Choose an available road agent."))?;
        let a = worker(&connection, &plan.worker_a)?;
        let b = worker(&connection, &plan.worker_b)?;
        planning::agent_plan(plan, &a, &b, &agent).map_err(|e| rule(e.to_string()))
    }

    pub fn save_segment(&self, request: SaveSegmentRequest) -> Result<ShowCard, PersistenceError> {
        let mut connection = self.career_connection(&request.save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let show = card(&tx, request.show_id)?;
        editable(&show, request.revision)?;
        validate_segment(&tx, &request.content)?;
        if request
            .segment_id
            .is_some_and(|id| !show.segments.iter().any(|s| s.id == id))
        {
            return Err(rule("The segment changed; reload the card."));
        }
        let duration = show
            .segments
            .iter()
            .filter(|s| Some(s.id) != request.segment_id)
            .map(|s| s.content.duration())
            .sum::<u32>()
            + request.content.duration();
        if duration > show.capacity_seconds
            || (request.segment_id.is_none() && show.segments.len() >= 20)
        {
            return Err(rule(
                "The card exceeds its two-hour broadcast window or twenty segments.",
            ));
        }
        if let SegmentPlan::Match(plan) = &request.content {
            for segment in show
                .segments
                .iter()
                .filter(|s| Some(s.id) != request.segment_id)
            {
                if let SegmentPlan::Match(other) = &segment.content
                    && [&other.worker_a, &other.worker_b]
                        .iter()
                        .any(|id| *id == &plan.worker_a || *id == &plan.worker_b)
                {
                    return Err(rule(
                        "A wrestler can work only one match on this card in the current engine.",
                    ));
                }
            }
        }
        let title = segment_title(&tx, &request.content)?;
        if let Some(id) = request.segment_id {
            tx.execute(
                "UPDATE segments SET title=?1,plan=?2 WHERE id=?3",
                params![title, encode(&request.content)?, id],
            )?;
        } else {
            tx.execute(
                "INSERT INTO segments(show_id,position,title,plan) VALUES(?1,?2,?3,?4)",
                params![
                    show.id,
                    show.segments.len() as i32,
                    title,
                    encode(&request.content)?
                ],
            )?;
        }
        tx.execute(
            "UPDATE shows SET revision=revision+1 WHERE id=?1",
            [show.id],
        )?;
        tx.commit()?;
        card(&connection, show.id)
    }

    pub fn rearrange_card(
        &self,
        save_id: &str,
        show_id: i32,
        revision: i32,
        ids: Vec<i32>,
    ) -> Result<ShowCard, PersistenceError> {
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let show = card(&tx, show_id)?;
        editable(&show, revision)?;
        if ids.len() != show.segments.len()
            || ids.iter().copied().collect::<BTreeSet<_>>()
                != show.segments.iter().map(|s| s.id).collect()
        {
            return Err(rule(
                "The running order must contain every segment exactly once.",
            ));
        }
        for (position, id) in ids.into_iter().enumerate() {
            tx.execute(
                "UPDATE segments SET position=?1 WHERE id=?2",
                params![position as i32, id],
            )?;
        }
        tx.execute(
            "UPDATE shows SET revision=revision+1 WHERE id=?1",
            [show_id],
        )?;
        tx.commit()?;
        card(&connection, show_id)
    }

    pub fn delete_segment(
        &self,
        save_id: &str,
        show_id: i32,
        revision: i32,
        segment_id: i32,
    ) -> Result<ShowCard, PersistenceError> {
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let show = card(&tx, show_id)?;
        editable(&show, revision)?;
        tx.execute(
            "DELETE FROM segments WHERE id=?1 AND show_id=?2",
            params![segment_id, show_id],
        )?;
        tx.execute(
            "UPDATE shows SET revision=revision+1 WHERE id=?1",
            [show_id],
        )?;
        tx.commit()?;
        card(&connection, show_id)
    }

    pub fn start_show(&self, save_id: &str, show_id: i32) -> Result<LiveView, PersistenceError> {
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let show = card(&tx, show_id)?;
        if show.status != "draft" {
            return live_view(&tx, show_id);
        }
        if show.segments.is_empty() {
            return Err(rule("Book at least one segment before going on air."));
        }
        let promotion = read_overview(&tx)?;
        if promotion.current_date != show.date {
            return Err(rule("Continue to the show's date before going on air."));
        }
        let mut ids = BTreeSet::new();
        for s in &show.segments {
            validate_segment(&tx, &s.content)?;
            match &s.content {
                SegmentPlan::Match(p) => {
                    ids.insert(p.worker_a.clone());
                    ids.insert(p.worker_b.clone());
                }
                SegmentPlan::Angle(p) => {
                    ids.extend(p.participants.clone());
                }
            }
        }
        let workers = ids
            .into_iter()
            .map(|id| worker(&tx, &id))
            .collect::<Result<Vec<_>, _>>()?;
        let costs = 180_000 + workers.iter().map(|w| w.appearance_fee).sum::<i32>();
        if promotion.cash_pence < i64::from(costs) {
            return Err(rule(
                "Insufficient funds to cover venue and guaranteed appearance costs.",
            ));
        }
        let relationships = tx
            .prepare(
                "SELECT worker_a,worker_b,chemistry FROM relationships ORDER BY worker_a,worker_b",
            )?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let trust = tx.query_row("SELECT trust FROM audience WHERE id=1", [], |r| r.get(0))?;
        let session = Session::new(
            promotion
                .seed
                .parse()
                .map_err(|_| PersistenceError::InvalidDatabase)?,
            show_id,
            show.segments,
            workers,
            agents(&tx)?,
            trust,
            relationships,
        )
        .map_err(|e| rule(e.to_string()))?;
        tx.execute(
            "INSERT INTO show_runtime(show_id,snapshot) VALUES(?1,?2)",
            params![show_id, encode(&session)?],
        )?;
        tx.execute("UPDATE shows SET status='live' WHERE id=?1", [show_id])?;
        tx.commit()?;
        live_view(&connection, show_id)
    }

    pub fn live_show(&self, save_id: &str, show_id: i32) -> Result<LiveView, PersistenceError> {
        live_view(&self.career_connection(save_id)?, show_id)
    }

    pub fn advance_show(&self, request: AdvanceRequest) -> Result<LiveView, PersistenceError> {
        if request.seconds == 0 || request.seconds > 3600 {
            return Err(rule("Advance between one second and one hour at a time."));
        }
        let mut connection = self.career_connection(&request.save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut session = get_session(&tx, request.show_id)?;
        if session.complete {
            return live_view(&tx, request.show_id);
        }
        if session.tick != request.expected_tick {
            return Err(rule(
                "The show has advanced in another request. Reload its live state.",
            ));
        }
        let previous_reports = session.reports.len();
        let events = session.advance(request.seconds);
        for event in events {
            tx.execute(
                "INSERT INTO simulation_events(show_id,sequence,data) VALUES(?1,?2,?3)",
                params![request.show_id, event.sequence, encode(&event)?],
            )?;
        }
        for report in session.reports.iter().skip(previous_reports) {
            for change in &report.changes {
                tx.execute("INSERT INTO worker_history(worker_id,show_id,segment_id,report) VALUES(?1,?2,?3,?4)",params![change.worker_id,request.show_id,report.segment_id,encode(report)?])?;
            }
            tx.execute("INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('segment_completed',?1,?2)",params![read_overview(&tx)?.current_date,encode(report)?])?;
        }
        if session.reports.len() > previous_reports {
            for w in &session.workers {
                write_worker(&tx, w)?;
            }
            for a in &session.agents {
                tx.execute(
                    "UPDATE agents SET data=?1 WHERE id=?2",
                    params![encode(a)?, a.id],
                )?;
            }
            for (a, b, value) in &session.relationships {
                tx.execute("INSERT INTO relationships(worker_a,worker_b,chemistry) VALUES(?1,?2,?3) ON CONFLICT(worker_a,worker_b) DO UPDATE SET chemistry=excluded.chemistry",params![a,b,value])?;
            }
        }
        tx.execute(
            "UPDATE show_runtime SET snapshot=?1 WHERE show_id=?2",
            params![encode(&session)?, request.show_id],
        )?;
        if session.complete {
            complete_show(&tx, &session)?;
        }
        tx.commit()?;
        live_view(&connection, request.show_id)
    }

    pub fn live_instruction(
        &self,
        save_id: &str,
        show_id: i32,
        instruction: LiveInstruction,
    ) -> Result<LiveView, PersistenceError> {
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut session = get_session(&tx, show_id)?;
        session
            .queue(instruction)
            .map_err(|e| rule(e.to_string()))?;
        tx.execute(
            "UPDATE show_runtime SET snapshot=?1 WHERE show_id=?2",
            params![encode(&session)?, show_id],
        )?;
        tx.commit()?;
        live_view(&connection, show_id)
    }

    pub fn show_report(&self, save_id: &str, show_id: i32) -> Result<ShowReport, PersistenceError> {
        get_report(&self.career_connection(save_id)?, show_id)?
            .ok_or_else(|| rule("The show has not finished yet."))
    }

    pub fn continue_day(&self, save_id: &str) -> Result<CareerOffice, PersistenceError> {
        let mut connection = self.career_connection(save_id)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let current = read_overview(&tx)?.current_date;
        let blocking: i32 = tx.query_row(
            "SELECT COUNT(*) FROM shows WHERE status='live' OR (status='draft' AND show_date<=?1)",
            [&current],
            |r| r.get(0),
        )?;
        if blocking > 0 {
            return Err(rule(
                "Today's show needs your attention. Book and complete it before continuing.",
            ));
        }
        let date = next_date(&current, 1)?;
        tx.execute("UPDATE promotions SET current_date=?1", [&date])?;
        super::identity::expire(&tx, &date)?;
        super::relationships::decay(&tx, &date)?;
        let ids = tx
            .prepare("SELECT id FROM workers ORDER BY id")?
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let birthday = &date[5..] == "01-01";
        for id in ids {
            let mut w = worker(&tx, &id)?;
            let cleared = w.condition.injury_days == 1;
            w.condition.fatigue = (w.condition.fatigue - 8 - w.attributes.sim_stamina() / 5).max(0);
            w.condition.injury_days = (w.condition.injury_days - 1).max(0);
            w.wrestling_style.register_day(&w.attributes);
            if birthday {
                w.age += 1;
                if w.age > 38 {
                    w.attributes.physicality.stamina =
                        w.attributes.physicality.stamina.adjusted(-1);
                }
            }
            write_worker(&tx, &w)?;
            if cleared {
                super::news::medical_clearance(&tx, &w, &date)?;
            }
        }
        tx.execute("INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('day_advanced',?1,'{}')",[&date])?;
        tx.commit()?;
        self.career_office(save_id)
    }
}

fn editable(show: &ShowCard, revision: i32) -> Result<(), PersistenceError> {
    if show.status != "draft" {
        return Err(rule("A show that has started cannot be rebooked."));
    }
    if show.revision != revision {
        return Err(rule(
            "The card has changed. Reload it before saving another edit.",
        ));
    }
    Ok(())
}
fn validate_segment(
    connection: &Connection,
    segment: &SegmentPlan,
) -> Result<(), PersistenceError> {
    match segment {
        SegmentPlan::Match(p) => {
            let agent = agents(connection)?
                .into_iter()
                .find(|a| a.id == p.agent_id)
                .ok_or_else(|| rule("Choose an available agent."))?;
            planning::validate_plan(
                p,
                &worker(connection, &p.worker_a)?,
                &worker(connection, &p.worker_b)?,
                &agent,
            )
            .map_err(|e| rule(e.to_string()))
        }
        SegmentPlan::Angle(p) => {
            if p.participants.is_empty()
                || p.participants.len() > 4
                || p.participants.iter().collect::<BTreeSet<_>>().len() != p.participants.len()
                || !(30..=600).contains(&p.duration_seconds)
                || p.purpose.trim().is_empty()
                || p.purpose.len() > 100
            {
                return Err(rule(
                    "An angle needs 1–4 distinct participants, a purpose and 30–600 seconds.",
                ));
            }
            for id in &p.participants {
                worker(connection, id)?;
            }
            Ok(())
        }
    }
}
fn segment_title(
    connection: &Connection,
    segment: &SegmentPlan,
) -> Result<String, PersistenceError> {
    Ok(match segment {
        SegmentPlan::Match(p) => format!(
            "{} vs {}",
            worker(connection, &p.worker_a)?.name,
            worker(connection, &p.worker_b)?.name
        ),
        SegmentPlan::Angle(p) => format!(
            "{} · {}",
            p.purpose,
            worker(connection, &p.participants[0])?.name
        ),
    })
}
fn complete_show(connection: &Connection, session: &Session) -> Result<(), PersistenceError> {
    let show = card(connection, session.show_id)?;
    if show.status == "complete" {
        return Ok(());
    }
    let popularity = session
        .workers
        .iter()
        .map(|w| w.condition.popularity)
        .sum::<i32>()
        / session.workers.len().max(1) as i32;
    let trust: i32 =
        connection.query_row("SELECT trust FROM audience WHERE id=1", [], |r| r.get(0))?;
    let attendance = (100 + trust * 5 + popularity * 4).clamp(80, 2500);
    let revenue = attendance * 2000;
    let costs = 180_000
        + session
            .workers
            .iter()
            .map(|w| w.appearance_fee)
            .sum::<i32>();
    let report = ShowReport {
        show_id: show.id,
        name: show.name,
        date: show.date.clone(),
        segments: session.reports.clone(),
        crowd: session.crowd.clone(),
        attendance,
        revenue_pence: revenue,
        costs_pence: costs,
    };
    connection.execute(
        "UPDATE shows SET status='complete',report=?1 WHERE id=?2",
        params![encode(&report)?, show.id],
    )?;
    connection.execute(
        "UPDATE promotions SET cash_pence=cash_pence+?1",
        [revenue - costs],
    )?;
    connection.execute("INSERT INTO ledger(show_id,amount_pence,reason) VALUES(?1,?2,'Gate less venue and appearance fees')",params![show.id,revenue-costs])?;
    connection.execute(
        "UPDATE audience SET trust=?1 WHERE id=1",
        [session.crowd.trust],
    )?;
    for (author, text) in media_for(&report) {
        connection.execute(
            "INSERT INTO media_posts(show_id,author,text,posted_on) VALUES(?1,?2,?3,?4)",
            params![show.id, author, text, show.date],
        )?;
    }
    connection.execute(
        "INSERT INTO domain_events(event_type,occurred_on,payload) VALUES('show_completed',?1,?2)",
        params![show.date, encode(&report)?],
    )?;
    super::news::publish_show(connection, &report)?;
    connection.execute("INSERT INTO shows(name,show_date,status,revision) VALUES('UWF Thursday Night',?1,'draft',0)",[next_date(&show.date,7)?])?;
    Ok(())
}
