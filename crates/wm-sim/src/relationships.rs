//! Deterministic relationship seeding and lightweight authored conversation resolution.

use wm_domain::game::Worker;
use wm_domain::identity::{PersonalityField, QualityField};
use wm_domain::relationships::{
    InteractionKind, InteractionOutcome, InteractionTone, ManagementRelationshipState,
    PersonalRelationshipState, RelationshipDelta, RelationshipMemory, RelationshipMemoryKind,
    RelationshipScores, interaction_effects,
};

#[derive(Debug, Clone)]
pub struct InteractionContext {
    pub request_id: String,
    pub occurred_on: String,
    pub recent_work: Option<String>,
    pub colleague: Option<(String, String, String)>,
    pub prior_count: u32,
}

#[derive(Debug, Clone)]
pub struct InteractionResolution {
    pub management: ManagementRelationshipState,
    pub outcome: InteractionOutcome,
}

pub fn initial_relationship(
    subject: &Worker,
    other: &Worker,
    date: &str,
) -> (PersonalRelationshipState, Option<RelationshipMemory>) {
    let mut state = default_relationship(&subject.id, &other.id, date);
    let shared_school = !subject.school.trim().is_empty() && subject.school == other.school;
    let before = state.scores;
    if shared_school {
        state.scores = state.scores.adjusted(14, 0, 12, -2);
    }
    let impact = RelationshipDelta {
        affinity: state.scores.affinity - before.affinity,
        respect: state.scores.respect - before.respect,
        trust: state.scores.trust - before.trust,
        tension: state.scores.tension - before.tension,
    };
    let memory = shared_school.then(|| RelationshipMemory {
        id: format!("shared-school:{}:{}", subject.id, other.id),
        subject_id: subject.id.clone(),
        other_id: other.id.clone(),
        occurred_on: date.into(),
        kind: RelationshipMemoryKind::SharedBackground,
        summary: format!(
            "A shared background at {} gives them some common ground.",
            subject.school
        ),
        impact,
        salience: 35,
        active_until: None,
        source: "world-generation-v1".into(),
    });
    (state, memory)
}

pub fn default_relationship(
    subject_id: &str,
    other_id: &str,
    date: &str,
) -> PersonalRelationshipState {
    let directional = stable_hash(&format!("{subject_id}>{other_id}"));
    let variation = |shift: u32, width: i32| ((directional >> shift) % width as u64) as i32;
    let affinity = variation(0, 21) - 10;
    let respect = variation(8, 17) - 8;
    let trust = variation(16, 17) - 8;
    let tension = (variation(24, 12) - affinity.max(0) / 8).max(0);
    let scores = RelationshipScores {
        affinity,
        respect,
        trust,
        tension,
    };
    PersonalRelationshipState {
        subject_id: subject_id.into(),
        other_id: other_id.into(),
        scores,
        revision: 0,
        updated_on: date.into(),
    }
}

pub fn initial_management(
    company_id: &str,
    worker_id: &str,
    date: &str,
) -> ManagementRelationshipState {
    ManagementRelationshipState {
        company_id: company_id.into(),
        worker_id: worker_id.into(),
        scores: RelationshipScores::default(),
        revision: 0,
        updated_on: date.into(),
    }
}

pub fn resolve_interaction(
    worker: &Worker,
    mut management: ManagementRelationshipState,
    kind: InteractionKind,
    context: InteractionContext,
) -> InteractionResolution {
    let p = |field| personality(worker, field);
    let q = |field| quality(worker, field);
    let relationship_pull =
        (management.scores.affinity + management.scores.trust) / 8 - management.scores.tension / 3;
    let disposition = match kind {
        InteractionKind::IntroduceYourself => {
            (p(PersonalityField::Sociability) + p(PersonalityField::Empathy)) / 2
        }
        InteractionKind::CheckIn => {
            (p(PersonalityField::Sociability) + p(PersonalityField::Temperament)) / 2
        }
        InteractionKind::PraiseRecentWork => {
            (p(PersonalityField::Ambition) + q(QualityField::Professionalism)) / 2
        }
        InteractionKind::OfferEncouragement => {
            (p(PersonalityField::Empathy) + p(PersonalityField::Temperament)) / 2
        }
        InteractionKind::AskForCreativeInput => {
            (p(PersonalityField::Outspokenness) + q(QualityField::Creativity)) / 2
        }
        InteractionKind::DiscussColleague => {
            (p(PersonalityField::Outspokenness) + p(PersonalityField::Sociability)) / 2
        }
        InteractionKind::ClearTheAir => {
            (p(PersonalityField::Empathy)
                + p(PersonalityField::Integrity)
                + p(PersonalityField::Temperament))
                / 3
        }
    };
    let score = 50 + (disposition - 50) / 2 + relationship_pull;
    let tone = match score {
        65.. => InteractionTone::Warm,
        48..=64 => InteractionTone::Open,
        34..=47 => InteractionTone::Guarded,
        _ => InteractionTone::Defensive,
    };
    let before = management.scores;
    let (affinity, respect, trust, tension, morale_change, confidence_change) = changes(kind, tone);
    management.scores = management
        .scores
        .adjusted(affinity, respect, trust, tension);
    let relationship_delta = RelationshipDelta {
        affinity: management.scores.affinity - before.affinity,
        respect: management.scores.respect - before.respect,
        trust: management.scores.trust - before.trust,
        tension: management.scores.tension - before.tension,
    };
    let morale_delta =
        (worker.condition.morale + morale_change).clamp(0, 100) - worker.condition.morale;
    let confidence_delta = (worker.condition.confidence + confidence_change).clamp(0, 100)
        - worker.condition.confidence;
    management.revision = management.revision.saturating_add(1);
    management.updated_on = context.occurred_on.clone();

    let response = response(worker, kind, tone, &context);
    let mut factors = vec![personality_factor(kind, disposition).into()];
    if management.scores.tension >= 35 {
        factors.push("Existing tension made the conversation harder.".into());
    } else if relationship_pull >= 8 {
        factors.push("An established rapport helped them speak openly.".into());
    }
    if let Some((name, summary, _)) = &context.colleague {
        factors.push(format!("Their view of {name} is currently: {summary}."));
    }
    let effects = interaction_effects(relationship_delta, morale_delta, confidence_delta);
    InteractionResolution {
        management,
        outcome: InteractionOutcome {
            request_id: context.request_id,
            worker_id: worker.id.clone(),
            kind,
            label: kind.label().into(),
            occurred_on: context.occurred_on,
            tone,
            response,
            factors,
            relationship_delta,
            morale_delta,
            confidence_delta,
            effects,
            context_worker_id: context.colleague.map(|(_, _, id)| id),
        },
    }
}

fn changes(kind: InteractionKind, tone: InteractionTone) -> (i32, i32, i32, i32, i32, i32) {
    let tier = match tone {
        InteractionTone::Warm => 3,
        InteractionTone::Open => 2,
        InteractionTone::Guarded => 1,
        InteractionTone::Defensive => 0,
    };
    match kind {
        InteractionKind::IntroduceYourself => [
            (-1, 0, -2, 2, 0, 0),
            (1, 0, 0, 0, 0, 0),
            (3, 1, 2, -1, 1, 0),
            (5, 1, 3, -2, 1, 0),
        ][tier],
        InteractionKind::CheckIn => [
            (-2, 0, -1, 2, -1, 0),
            (0, 0, 0, 0, 0, 0),
            (2, 0, 1, -1, 1, 0),
            (4, 1, 2, -2, 2, 0),
        ][tier],
        InteractionKind::PraiseRecentWork => [
            (0, -1, 0, 1, -1, 0),
            (1, 0, 0, 0, 1, 0),
            (2, 1, 1, -1, 2, 0),
            (3, 2, 1, -1, 3, 0),
        ][tier],
        InteractionKind::OfferEncouragement => [
            (-2, 0, -2, 2, -2, -2),
            (0, 0, 0, 0, 1, 1),
            (2, 0, 1, -1, 2, 2),
            (4, 1, 3, -2, 4, 4),
        ][tier],
        InteractionKind::AskForCreativeInput => [
            (-1, -1, -2, 2, -1, 0),
            (1, 0, 0, 0, 0, 0),
            (2, 1, 2, -1, 1, 0),
            (4, 2, 3, -2, 2, 0),
        ][tier],
        InteractionKind::DiscussColleague => [
            (-1, 0, -1, 1, 0, 0),
            (0, 0, 0, 0, 0, 0),
            (1, 0, 1, 0, 0, 0),
            (2, 1, 2, -1, 1, 0),
        ][tier],
        InteractionKind::ClearTheAir => [
            (-2, 0, -3, 4, -1, 0),
            (0, 0, 0, -2, 0, 0),
            (2, 1, 2, -7, 1, 0),
            (3, 2, 4, -12, 2, 0),
        ][tier],
    }
}

fn response(
    worker: &Worker,
    kind: InteractionKind,
    tone: InteractionTone,
    context: &InteractionContext,
) -> String {
    let first = worker
        .name
        .split_whitespace()
        .next()
        .unwrap_or(&worker.name);
    let variant =
        (stable_hash(&format!("{}:{kind:?}:{}", worker.id, context.prior_count)) % 3) as usize;
    let recent = context.recent_work.as_deref().unwrap_or("the recent show");
    let colleague = context
        .colleague
        .as_ref()
        .map(|(name, summary, _)| format!("As for {name}, I'd describe it as {summary}."))
        .unwrap_or_default();
    let lines: [&str; 3] = match (kind, tone) {
        (InteractionKind::IntroduceYourself, InteractionTone::Warm) => [
            "I'm glad we finally sat down. Tell me what you need from me and I'll be straight with you.",
            "Good to meet properly. I want this to work, so keep the communication honest.",
            "I've been looking forward to this. Let's make something worthwhile together.",
        ],
        (InteractionKind::IntroduceYourself, InteractionTone::Open) => [
            "Good to meet properly. I'm ready to work.",
            "I appreciate you making the time. Let's see how things develop.",
            "Thanks for sitting down with me. I'm listening.",
        ],
        (InteractionKind::IntroduceYourself, InteractionTone::Guarded) => [
            "All right. I'll let the work speak first.",
            "Good to meet you. I prefer to see how people operate before saying too much.",
            "Understood. We can take it one show at a time.",
        ],
        (InteractionKind::IntroduceYourself, InteractionTone::Defensive) => [
            "Introductions are fine. I don't think a speech changes much.",
            "Fine. Just be clear about what you expect.",
            "I'll hear you out, but trust takes more than a meeting.",
        ],
        (InteractionKind::CheckIn, InteractionTone::Warm) => [
            "Honestly, I'm doing well. It means something that you asked.",
            "I'm good, and I appreciate the check-in. I'll come to you if that changes.",
            "Thanks for treating me like a person, not a slot on the card. I appreciate the check-in.",
        ],
        (InteractionKind::CheckIn, InteractionTone::Open) => [
            "I'm doing all right. There are a couple of things on my mind, but nothing urgent.",
            "I'm okay. I appreciate you checking rather than assuming.",
            "Steady enough. I'll tell you if I need something.",
        ],
        (InteractionKind::CheckIn, InteractionTone::Guarded) => [
            "I'm fine. I'd rather focus on the work.",
            "Nothing I need to discuss right now.",
            "I'm handling it. Thanks.",
        ],
        (InteractionKind::CheckIn, InteractionTone::Defensive) => [
            "Why—has somebody said something about me?",
            "I'm here and I'm working. That's all you need to know.",
            "If there's a problem, just ask me directly.",
        ],
        (InteractionKind::PraiseRecentWork, InteractionTone::Warm) => [
            "That means a lot. I felt locked in out there.",
            "Thank you. I was proud of that one and I'm glad you noticed.",
            "I appreciate it. Give me another opportunity like that and I'll build on it.",
        ],
        (InteractionKind::PraiseRecentWork, InteractionTone::Open) => [
            "Thanks. I thought it came together well.",
            "I appreciate that. There are still things I can sharpen.",
            "Good to hear. I'll keep pushing.",
        ],
        (InteractionKind::PraiseRecentWork, InteractionTone::Guarded) => [
            "Thanks. I was just doing my job.",
            "Noted. Let's see what comes next.",
            "I appreciate it, though I know it wasn't perfect.",
        ],
        (InteractionKind::PraiseRecentWork, InteractionTone::Defensive) => [
            "Is there a catch?",
            "Thanks, but one compliment doesn't fix everything.",
            "I know what I delivered. I hope the booking reflects it.",
        ],
        (InteractionKind::OfferEncouragement, InteractionTone::Warm) => [
            "I needed that more than I realised. Thank you.",
            "That helps. I'll reset and come back stronger.",
            "I appreciate you having my back. I won't waste it.",
        ],
        (InteractionKind::OfferEncouragement, InteractionTone::Open) => [
            "Thanks. I'll keep working through it.",
            "I appreciate that. It's been a difficult stretch.",
            "Understood. I'll focus on what I can control.",
        ],
        (InteractionKind::OfferEncouragement, InteractionTone::Guarded) => [
            "I hear you. I need results more than reassurance right now.",
            "Thanks, but I'll feel better when things actually change.",
            "I appreciate the thought. Give me some space to work.",
        ],
        (InteractionKind::OfferEncouragement, InteractionTone::Defensive) => [
            "I don't need a pep talk.",
            "If you want to help, change the situation.",
            "Words are easy. Let me prove myself in the ring.",
        ],
        (InteractionKind::AskForCreativeInput, InteractionTone::Warm) => [
            "I've got a few ideas. Let me walk you through the strongest one.",
            "Absolutely. I think we can make this feel more personal without forcing it.",
            "Glad you asked. I have a direction that fits who I am and gives the crowd something clear.",
        ],
        (InteractionKind::AskForCreativeInput, InteractionTone::Open) => [
            "I have some thoughts. Nothing is precious—let's shape them together.",
            "Yes. I think a small adjustment could make the presentation click.",
            "I'll put something concise together and bring it to you.",
        ],
        (InteractionKind::AskForCreativeInput, InteractionTone::Guarded) => [
            "I can suggest something, but I need to know you'll actually consider it.",
            "Maybe. What part are you genuinely willing to change?",
            "I've got an idea, though I'd rather not oversell it yet.",
        ],
        (InteractionKind::AskForCreativeInput, InteractionTone::Defensive) => [
            "Now you want my input?",
            "I'll share an idea, but I need to know you're genuinely listening.",
            "I can answer, but I won't pretend to have creative control that I don't have.",
        ],
        (InteractionKind::DiscussColleague, InteractionTone::Warm) => [
            "Sure. I'll be candid, but I don't want private feedback turned into gossip.",
            "I can talk about them fairly. I'll be specific about how I see it.",
            "Of course. I'll tell you what works between us and where it gets difficult.",
        ],
        (InteractionKind::DiscussColleague, InteractionTone::Open) => [
            "I'll give you an honest professional view.",
            "All right. I can explain how I see the relationship.",
            "I don't mind discussing it privately.",
        ],
        (InteractionKind::DiscussColleague, InteractionTone::Guarded) => [
            "I'd rather keep it professional, but I'll answer carefully.",
            "I can say a little. I don't want this becoming a locker-room story.",
            "That's between us, right? Then I'll give you the short version.",
        ],
        (InteractionKind::DiscussColleague, InteractionTone::Defensive) => [
            "I'm not talking about a colleague behind their back.",
            "Why are you asking me instead of them?",
            "No. I don't want my words used against either of us.",
        ],
        (InteractionKind::ClearTheAir, InteractionTone::Warm) => [
            "You're right—we needed this. I don't want the tension carrying into the work.",
            "I appreciate you addressing it directly. We can move forward.",
            "Thank you for owning the conversation. Let's reset.",
        ],
        (InteractionKind::ClearTheAir, InteractionTone::Open) => [
            "I won't pretend everything is fixed, but this is a start.",
            "All right. I can meet you halfway and see where it goes.",
            "I appreciate the directness. Let's judge it by what happens next.",
        ],
        (InteractionKind::ClearTheAir, InteractionTone::Guarded) => [
            "I've heard you. I need time and consistent action now.",
            "We can leave it there for today. I'm not ready to call it resolved.",
            "That's noted. Trust will take longer.",
        ],
        (InteractionKind::ClearTheAir, InteractionTone::Defensive) => [
            "No. This feels like you're trying to close the issue without fixing it.",
            "I'm not ready to move past it just because we had a meeting.",
            "If you want this resolved, the behaviour has to change first.",
        ],
    };
    let mut text = lines[variant].replace("the recent show", recent);
    if kind == InteractionKind::PraiseRecentWork && !text.contains(recent) {
        text = format!("About {recent}: {text}");
    }
    if kind == InteractionKind::DiscussColleague && !colleague.is_empty() {
        text = format!("{text} {colleague}");
    }
    format!("{first}: \"{text}\"")
}

fn personality(worker: &Worker, field: PersonalityField) -> i32 {
    worker
        .identity
        .personality
        .get(&field)
        .and_then(|a| a.value)
        .map(|v| v.get())
        .unwrap_or(50)
}

fn quality(worker: &Worker, field: QualityField) -> i32 {
    worker
        .identity
        .qualities
        .get(&field)
        .and_then(|a| a.value)
        .map(|v| v.get())
        .unwrap_or(50)
}

fn personality_factor(kind: InteractionKind, disposition: i32) -> &'static str {
    match (kind, disposition) {
        (InteractionKind::AskForCreativeInput, 65..) => {
            "Their creativity and candour helped them engage with the question."
        }
        (InteractionKind::DiscussColleague, 65..) => {
            "Their outgoing, candid nature made them more willing to talk."
        }
        (InteractionKind::ClearTheAir, 65..) => {
            "Their empathy and composure helped them address the disagreement."
        }
        (_, 65..) => "Their personal disposition made them receptive to this approach.",
        (_, ..=34) => "Their personal disposition made this approach uncomfortable for them.",
        _ => "Their personal disposition neither strongly helped nor hurt the conversation.",
    }
}

fn stable_hash(value: &str) -> u64 {
    value.bytes().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{base_pack, generate_world};

    #[test]
    fn initial_relationships_are_directional_bounded_and_deterministic() {
        let workers = generate_world(42, &base_pack());
        let first = initial_relationship(&workers[0], &workers[1], "2026-01-01");
        assert_eq!(
            first,
            initial_relationship(&workers[0], &workers[1], "2026-01-01")
        );
        first.0.validate().unwrap();
        let reverse = initial_relationship(&workers[1], &workers[0], "2026-01-01");
        assert_ne!(first.0.scores, reverse.0.scores);
    }

    #[test]
    fn conversation_resolution_is_repeatable_and_bounded() {
        let worker = generate_world(42, &base_pack()).remove(0);
        let management = initial_management("uwf", &worker.id, "2026-01-01");
        let context = InteractionContext {
            request_id: "request-1".into(),
            occurred_on: "2026-01-01".into(),
            recent_work: None,
            colleague: None,
            prior_count: 0,
        };
        let a = resolve_interaction(
            &worker,
            management.clone(),
            InteractionKind::CheckIn,
            context.clone(),
        );
        let b = resolve_interaction(&worker, management, InteractionKind::CheckIn, context);
        assert_eq!(a.outcome, b.outcome);
        a.management.validate().unwrap();
    }
}
