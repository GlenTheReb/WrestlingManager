use super::*;
use rusqlite::TransactionBehavior;
use wm_domain::{
    game::{NewsItem, NewsPage, ShowReport, Worker},
    relationships::InteractionOutcome,
};

// Articles are a projection of recorded career events. The source key makes projection idempotent.
fn publish(db: &Connection, source: &str, item: NewsItem) -> Result<(), PersistenceError> {
    db.execute("INSERT INTO news_items(source_key,category,title,body,occurred_on,show_id,worker_id) VALUES(?1,?2,?3,?4,?5,?6,?7) ON CONFLICT(source_key) DO NOTHING",
        params![source,item.category,item.title,item.body,item.date,item.show_id,item.worker_id])?;
    Ok(())
}

pub(super) fn publish_interaction(
    db: &Connection,
    worker_name: &str,
    outcome: &InteractionOutcome,
) -> Result<(), PersistenceError> {
    publish(
        db,
        &format!("interaction:{}", outcome.request_id),
        article(
            "People",
            format!("Conversation with {worker_name}"),
            format!(
                "{}\n\n{}\n\nStaff note: {}",
                outcome.label,
                outcome.response,
                outcome.effects.join(" ")
            ),
            &outcome.occurred_on,
            None,
            Some(outcome.worker_id.clone()),
        ),
    )
}

fn article(
    category: &str,
    title: String,
    body: String,
    date: &str,
    show_id: Option<i32>,
    worker_id: Option<String>,
) -> NewsItem {
    NewsItem {
        id: 0,
        category: category.into(),
        title,
        body,
        date: date.into(),
        show_id,
        worker_id,
        read: false,
    }
}

pub(super) fn migrate(connection: &mut Connection) -> Result<(), PersistenceError> {
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let version: u32 = tx.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if version >= 3 {
        return Ok(());
    }
    if version != 2 {
        return Err(PersistenceError::InvalidDatabase);
    }
    tx.execute_batch(include_str!("../migrations/003_news.sql"))?;
    let promotion = read_overview(&tx)?;
    let next_show: Option<i32> = tx
        .query_row(
            "SELECT id FROM shows WHERE status='draft' ORDER BY id LIMIT 1",
            [],
            |r| r.get(0),
        )
        .optional()?;
    let count: i32 = tx.query_row("SELECT COUNT(*) FROM workers", [], |r| r.get(0))?;
    publish(
        &tx,
        "office:briefing",
        article(
            "Office",
            format!("The book is yours at {}", promotion.initials),
            format!(
                "You are directing {} in {}. There are {count} wrestlers on the roster. Review their condition, appoint a road agent and prepare your next running order. This desk collects confirmed events from your career as they happen.",
                promotion.name, promotion.region
            ),
            &promotion.current_date,
            next_show,
            None,
        ),
    )?;
    // Backfill actual completed-show reports; never invent incidents for an existing career.
    let mut statement = tx.prepare(
        "SELECT report FROM shows WHERE status='complete' AND report IS NOT NULL ORDER BY id",
    )?;
    let reports = statement.query_map([], |r| r.get::<_, String>(0))?;
    for json in reports {
        let report: ShowReport =
            serde_json::from_str(&json?).map_err(|_| PersistenceError::InvalidDatabase)?;
        publish_show(&tx, &report)?;
    }
    drop(statement);
    tx.execute(
        "UPDATE metadata SET value='3' WHERE key='schema_version'",
        [],
    )?;
    tx.pragma_update(None, "user_version", 3)?;
    tx.commit()?;
    Ok(())
}

pub(super) fn publish_show(db: &Connection, report: &ShowReport) -> Result<(), PersistenceError> {
    let results = report
        .segments
        .iter()
        .map(|segment| format!("{}: {}.", segment.title, segment.result))
        .collect::<Vec<_>>()
        .join("\n\n");
    publish(
        db,
        &format!("show:{}:results", report.show_id),
        article(
            "Results",
            format!("{}: the night in full", report.name),
            format!(
                "{results}\n\nFinal crowd energy: {}. Audience trust: {}. The staff report contains execution, safety and development detail for every segment.",
                report.crowd.energy, report.crowd.trust
            ),
            &report.date,
            Some(report.show_id),
            None,
        ),
    )?;
    let net = report.revenue_pence - report.costs_pence;
    publish(
        db,
        &format!("show:{}:business", report.show_id),
        article(
            "Business",
            format!("{} attend {}", report.attendance, report.name),
            format!(
                "The gate brought in {} against {} in venue and appearance costs. The show returned {} {} to the company accounts.",
                pounds(report.revenue_pence),
                pounds(report.costs_pence),
                pounds(net.abs()),
                if net >= 0 { "profit" } else { "loss" }
            ),
            &report.date,
            Some(report.show_id),
            None,
        ),
    )?;
    for segment in &report.segments {
        for change in &segment.changes {
            let incident: bool = change.injury_days > 0 && db.query_row(
                "SELECT EXISTS(SELECT 1 FROM simulation_events WHERE show_id=?1 AND json_extract(data,'$.kind')='injury' AND json_extract(data,'$.actorId')=?2)",
                params![report.show_id,change.worker_id], |r| r.get(0))?;
            if incident {
                publish(
                    db,
                    &format!("show:{}:medical:{}", report.show_id, change.worker_id),
                    article(
                        "Medical",
                        format!("{} needs time out", change.name),
                        format!(
                            "After {}, the medical assessment records {} days before clearance. {} Open the wrestler's profile for their current availability.",
                            segment.title, change.injury_days, change.note
                        ),
                        &report.date,
                        Some(report.show_id),
                        Some(change.worker_id.clone()),
                    ),
                )?;
            }
        }
    }
    Ok(())
}

fn pounds(pence: i32) -> String {
    format!("£{}.{:02}", pence / 100, pence.abs() % 100)
}

pub(super) fn medical_clearance(
    db: &Connection,
    worker: &Worker,
    date: &str,
) -> Result<(), PersistenceError> {
    publish(
        db,
        &format!("clearance:{date}:{}", worker.id),
        article(
            "Medical",
            format!("{} is cleared to return", worker.name),
            format!(
                "{} has completed the recorded recovery period and is available for match booking. Current fatigue is {}/100; medical clearance does not mean they are fully rested.",
                worker.name, worker.condition.fatigue
            ),
            date,
            None,
            Some(worker.id.clone()),
        ),
    )
}

impl SaveRepository {
    pub fn news_page(
        &self,
        save_id: &str,
        category: &str,
        unread_only: bool,
        offset: u32,
        limit: u32,
    ) -> Result<NewsPage, PersistenceError> {
        let db = self.career_connection(save_id)?;
        if !["", "Office", "People", "Results", "Medical", "Business"].contains(&category) {
            return Err(PersistenceError::GameRule(
                "Choose a supported news category.".into(),
            ));
        }
        let unread = db.query_row("SELECT COUNT(*) FROM news_items WHERE is_read=0", [], |r| {
            r.get(0)
        })?;
        let total = db.query_row(
            "SELECT COUNT(*) FROM news_items WHERE (?1='' OR category=?1) AND (?2=0 OR is_read=0)",
            params![category, unread_only],
            |r| r.get(0),
        )?;
        let items = db.prepare("SELECT id,category,title,body,occurred_on,show_id,worker_id,is_read FROM news_items WHERE (?1='' OR category=?1) AND (?2=0 OR is_read=0) ORDER BY occurred_on DESC,id DESC LIMIT ?3 OFFSET ?4")?
            .query_map(params![category,unread_only,limit.clamp(1,50),offset], |r| Ok(NewsItem { id:r.get(0)?,category:r.get(1)?,title:r.get(2)?,body:r.get(3)?,date:r.get(4)?,show_id:r.get(5)?,worker_id:r.get(6)?,read:r.get(7)? }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(NewsPage {
            items,
            total,
            unread,
        })
    }

    pub fn set_news_read(
        &self,
        save_id: &str,
        id: i32,
        read: bool,
    ) -> Result<(), PersistenceError> {
        let db = self.career_connection(save_id)?;
        if db.execute(
            "UPDATE news_items SET is_read=?1 WHERE id=?2",
            params![read, id],
        )? != 1
        {
            return Err(PersistenceError::GameRule(
                "This news item is unavailable.".into(),
            ));
        }
        Ok(())
    }
}
