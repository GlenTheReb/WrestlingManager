use thiserror::Error;
use wm_domain::game::*;
use wm_domain::match_rules::{MatchDefinition, ParticipationRule, VictoryRule};

#[derive(Debug, Error)]
#[error("{0}")]
pub struct PlanError(pub String);

/// Structural validity is distinct from engine support. Future bookers can use
/// the domain contract without accidentally running team matches as singles.
pub fn validate_engine_support(definition: &MatchDefinition) -> Result<(), PlanError> {
    definition
        .validate()
        .map_err(|e| PlanError(e.to_string()))?;
    if definition.slots.len() != 2
        || definition.sides.len() != 2
        || definition.rules.participation != ParticipationRule::AllActive
        || definition.rules.victory != VictoryRule::OneFall
    {
        return Err(PlanError(
            "This booking is structurally valid, but tag, team, multi-person and elimination simulation is not available yet.".into(),
        ));
    }
    Ok(())
}

pub fn validate_plan(
    plan: &MatchPlan,
    a: &Worker,
    b: &Worker,
    agent: &RoadAgent,
) -> Result<(), PlanError> {
    let invalid = |s: &str| Err(PlanError(s.into()));
    if plan.worker_a != a.id || plan.worker_b != b.id || a.id == b.id {
        return invalid("Choose two different available wrestlers.");
    }
    if a.condition.injury_days > 0 || b.condition.injury_days > 0 {
        return invalid("A participant is not medically cleared.");
    }
    if agent.id != plan.agent_id {
        return invalid("The assigned road agent is unavailable.");
    }
    let definition = MatchDefinition::try_from(plan).map_err(|e| PlanError(e.to_string()))?;
    validate_engine_support(&definition)?;
    if plan
        .protected_worker_id
        .as_ref()
        .is_some_and(|id| id != &a.id && id != &b.id)
    {
        return invalid("Protection must refer to a participant.");
    }
    if !(120..=3600).contains(&plan.duration_seconds)
        || !(1..=5).contains(&plan.pace)
        || !(1..=5).contains(&plan.risk)
        || !(0..=100).contains(&plan.freedom)
        || plan.style.trim().is_empty()
        || plan.style.len() > 40
        || plan.purpose.trim().is_empty()
        || plan.purpose.len() > 160
        || plan.beats.len() > 60
    {
        return invalid(
            "Use 2–60 minutes, pace/risk 1–5, freedom 0–100 and a short match purpose (up to 60 beats).",
        );
    }
    let mut last = 0;
    for (i, beat) in plan.beats.iter().enumerate() {
        if beat.at_second < 10
            || beat.at_second >= plan.duration_seconds - 5
            || (i > 0 && beat.at_second <= last)
        {
            return invalid(
                "Sequence beats must have distinct, increasing times between the opening and booked finish.",
            );
        }
        last = beat.at_second;
        let worker = if beat.actor_id == a.id {
            a
        } else if beat.actor_id == b.id {
            b
        } else {
            return invalid("A sequence actor must be in the match.");
        };
        if beat.kind == BeatKind::Move
            && !worker
                .moves
                .iter()
                .any(|m| Some(&m.id) == beat.move_id.as_ref())
        {
            return invalid("Choose a move from that wrestler's own repertoire.");
        }
        if beat.kind != BeatKind::Move && beat.move_id.is_some() {
            return invalid("Only move beats can reference a move.");
        }
        if beat.kind == BeatKind::Control && !(5..=300).contains(&beat.duration_seconds) {
            return invalid("A control period must last 5–300 seconds.");
        }
        if beat.kind == BeatKind::Weapon
            && plan.match_type == "Singles"
            && plan.finish != Finish::Disqualification
            && !plan.beats[..i]
                .iter()
                .any(|beat| beat.kind == BeatKind::RefBump)
        {
            return invalid(
                "A weapon in a singles match needs a prior referee bump or a disqualification finish.",
            );
        }
    }
    Ok(())
}

pub fn agent_plan(
    mut plan: MatchPlan,
    a: &Worker,
    b: &Worker,
    agent: &RoadAgent,
) -> Result<AgentAdvice, PlanError> {
    validate_plan(&plan, a, b, agent)?;
    let mut notes = vec![format!("{}: {}", agent.name, agent.philosophy)];
    let preserve = plan.beats.len();
    for (fraction, worker) in [
        (12, a),
        (32, b),
        (56, a),
        (75, b),
        (
            90,
            if plan.winner_id.as_ref() == Some(&b.id) {
                b
            } else {
                a
            },
        ),
    ] {
        if plan.beats.len() >= 60 {
            break;
        }
        let at_second = plan.duration_seconds * fraction / 100;
        if plan
            .beats
            .iter()
            .any(|beat| beat.at_second.abs_diff(at_second) < 10)
        {
            continue;
        }
        let suitable: Vec<_> = worker
            .moves
            .iter()
            .filter(|m| {
                m.min_strength <= worker.attributes.sim_strength()
                    && (agent.safety < 14 || m.risk <= plan.risk)
                    && (fraction < 80 || agent.psychology < 14 || m.stamina_cost < 6)
            })
            .collect();
        let chosen = if fraction >= 80 {
            suitable.iter().find(|m| m.signature).copied()
        } else {
            None
        }
        .or_else(|| {
            suitable
                .get((fraction as usize / 10) % suitable.len().max(1))
                .copied()
        });
        if let Some(m) = chosen {
            plan.beats.push(PlannedBeat {
                at_second,
                actor_id: worker.id.clone(),
                kind: BeatKind::Move,
                move_id: Some(m.id.clone()),
                duration_seconds: 0,
            });
        }
    }
    plan.beats.sort_by_key(|b| b.at_second);
    notes.push(format!(
        "Kept {preserve} player beats; added {} linked exchanges with a closing payoff.",
        plan.beats.len() - preserve
    ));
    if a.condition.fatigue > 35 || b.condition.fatigue > 35 {
        notes.push(
            "One wrestler is carrying fatigue. Lower the pace or leave room for recovery holds."
                .into(),
        );
    }
    if plan.duration_seconds > 900 && plan.pace >= 4 {
        notes.push("A long match at this pace risks an exhausted closing stretch.".into());
    }
    if a.style != plan.style && b.style != plan.style {
        notes.push(
            "Neither wrestler specialises in this style. The agent will lean on familiar moves."
                .into(),
        );
    }
    if plan.protected_worker_id.is_some() {
        notes.push(
            "Protection will shape control, near-falls and the post-match presentation.".into(),
        );
    }
    validate_plan(&plan, a, b, agent)?;
    Ok(AgentAdvice { plan, notes })
}
