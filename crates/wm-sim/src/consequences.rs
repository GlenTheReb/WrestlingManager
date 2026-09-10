use crate::bounded;
use wm_domain::game::*;

pub fn apply_match(
    worker: &mut Worker,
    plan: &MatchPlan,
    performance: &Performance,
    remaining_stamina: i32,
    injured: bool,
    opponent_psychology: i32,
    used_moves: &[(String, String, u32)],
) -> WorkerChange {
    let before = worker.condition.clone();
    let won = plan.winner_id.as_ref() == Some(&worker.id);
    let protected = plan.protected_worker_id.as_ref() == Some(&worker.id);
    let minutes = plan.duration_seconds as i32 / 60;
    let c = &mut worker.condition;
    c.fatigue = bounded(c.fatigue + (1000 - remaining_stamina) / 18 + minutes / 2);
    c.confidence = bounded(
        c.confidence
            + if won {
                4
            } else if protected {
                0
            } else {
                -2
            }
            + if performance.execution > 70 { 2 } else { -2 },
    );
    c.morale = bounded(
        c.morale
            + if protected || won {
                2
            } else if performance.safety < 60 {
                -4
            } else {
                0
            },
    );
    c.momentum = bounded(
        c.momentum
            + if won {
                5
            } else if protected {
                -1
            } else {
                -3
            },
    );
    c.popularity = bounded(
        c.popularity
            + if performance.engagement > 70 {
                2
            } else if performance.engagement < 35 {
                -2
            } else {
                0
            },
    );
    c.wear = bounded(c.wear + minutes / 8 + plan.risk / 2 + i32::from(worker.age >= 38) * 2);
    c.injury_days = if injured {
        7 + (100 - performance.safety) / 8
    } else {
        c.injury_days
    };
    let learning = if worker.age < 25 {
        6 + minutes / 3 + opponent_psychology / 4
    } else if worker.age < 36 {
        3 + minutes / 5
    } else {
        1 + minutes / 10
    };
    c.development += learning;
    c.matches += 1;
    worker.wrestling_style.register_match(&worker.attributes);
    if c.development >= 100 {
        c.development -= 100;
        worker.attributes.psychology.match_storytelling =
            worker.attributes.psychology.match_storytelling.adjusted(1);
        if worker.age < 30 {
            worker.attributes.ringcraft.technical_grappling =
                worker.attributes.ringcraft.technical_grappling.adjusted(1);
        }
    }
    for m in &mut worker.moves {
        if let Some((_, _, uses)) = used_moves
            .iter()
            .find(|(id, mv, _)| id == &worker.id && mv == &m.id)
        {
            m.proficiency = (m.proficiency
                + (*uses as i32 / 3).min(if worker.age < 25 { 3 } else { 1 }))
            .min(100);
        }
    }
    WorkerChange {
        worker_id: worker.id.clone(),
        name: worker.name.clone(),
        fatigue: c.fatigue - before.fatigue,
        confidence: c.confidence - before.confidence,
        morale: c.morale - before.morale,
        momentum: c.momentum - before.momentum,
        popularity: c.popularity - before.popularity,
        development: learning,
        wear: c.wear - before.wear,
        injury_days: c.injury_days,
        note: if injured {
            "Medical assessment required after a physical incident."
        } else if worker.age < 25 {
            "A young performer gains experience, timing and familiarity with the moves used."
        } else if worker.age >= 38 {
            "The veteran gains some familiarity, but recovery and accumulated wear matter more."
        } else {
            "Ring experience and repeated moves contribute to steady development."
        }
        .into(),
    }
}

pub fn media_for(report: &ShowReport) -> Vec<(String, String)> {
    let mut posts = Vec::new();
    for segment in &report.segments {
        posts.push((
            "Ringside Wire".into(),
            format!("{}: {}. {}", report.name, segment.title, segment.result),
        ));
        if segment.performance.execution < 55 {
            posts.push((
                "The Front Row".into(),
                format!(
                    "{} never quite clicked tonight. {}",
                    segment.title,
                    segment.reasons.first().cloned().unwrap_or_default()
                ),
            ));
        } else if segment.performance.engagement > 70 {
            posts.push(("Bell to Bell".into(),format!("Still thinking about {}. The crowd found another gear in that closing stretch.",segment.title)));
        }
        for change in &segment.changes {
            if change.injury_days > 0 {
                posts.push(("Ringside Wire".into(),format!("{} is being assessed after tonight's match and is unavailable for upcoming bookings.",change.name)));
            }
        }
    }
    if report.crowd.interference_count > 1 {
        posts.push((
            "Terrace Talk".into(),
            "Another interference finish? The interruptions became the story of the night.".into(),
        ));
    }
    posts
}
