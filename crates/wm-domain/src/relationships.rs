//! Directional personal relationships and auditable management conversations.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const RELATIONSHIP_RULE_VERSION: u32 = 1;
pub const DAILY_INTERACTION_ATTENTION: u32 = 4;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RelationshipScores {
    pub affinity: i32,
    pub respect: i32,
    pub trust: i32,
    pub tension: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RelationshipDelta {
    pub affinity: i32,
    pub respect: i32,
    pub trust: i32,
    pub tension: i32,
}

impl RelationshipDelta {
    pub fn validate(&self) -> Result<(), String> {
        if [self.affinity, self.respect, self.trust, self.tension]
            .into_iter()
            .any(|value| !(-100..=100).contains(&value))
        {
            return Err("Relationship change is outside its valid bounds.".into());
        }
        Ok(())
    }
}

impl RelationshipScores {
    pub fn validate(&self) -> Result<(), String> {
        if !(-100..=100).contains(&self.affinity)
            || !(-100..=100).contains(&self.respect)
            || !(-100..=100).contains(&self.trust)
            || !(0..=100).contains(&self.tension)
        {
            return Err("Relationship scores are outside their valid bounds.".into());
        }
        Ok(())
    }

    pub fn adjusted(self, affinity: i32, respect: i32, trust: i32, tension: i32) -> Self {
        Self {
            affinity: (self.affinity + affinity).clamp(-100, 100),
            respect: (self.respect + respect).clamp(-100, 100),
            trust: (self.trust + trust).clamp(-100, 100),
            tension: (self.tension + tension).clamp(0, 100),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersonalRelationshipState {
    pub subject_id: String,
    pub other_id: String,
    pub scores: RelationshipScores,
    pub revision: u32,
    pub updated_on: String,
}

impl PersonalRelationshipState {
    pub fn validate(&self) -> Result<(), String> {
        if self.subject_id.trim().is_empty()
            || self.other_id.trim().is_empty()
            || self.subject_id == self.other_id
            || !valid_date(&self.updated_on)
        {
            return Err("Invalid directional relationship identity or date.".into());
        }
        self.scores.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ManagementRelationshipState {
    pub company_id: String,
    pub worker_id: String,
    pub scores: RelationshipScores,
    pub revision: u32,
    pub updated_on: String,
}

impl ManagementRelationshipState {
    pub fn validate(&self) -> Result<(), String> {
        if self.company_id.trim().is_empty()
            || self.worker_id.trim().is_empty()
            || !valid_date(&self.updated_on)
        {
            return Err("Invalid management relationship identity or date.".into());
        }
        self.scores.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipMemoryKind {
    SharedBackground,
    PositiveExperience,
    Disagreement,
    Betrayal,
    Support,
    Reconciliation,
    ManagementConversation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RelationshipMemory {
    pub id: String,
    pub subject_id: String,
    pub other_id: String,
    pub occurred_on: String,
    pub kind: RelationshipMemoryKind,
    pub summary: String,
    pub impact: RelationshipDelta,
    pub salience: u32,
    pub active_until: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersonalRelationshipEvent {
    pub memory: RelationshipMemory,
}

impl PersonalRelationshipEvent {
    pub fn validate(&self) -> Result<(), String> {
        self.memory.validate()
    }
}

impl RelationshipMemory {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty()
            || self.id.len() > 120
            || self.subject_id.trim().is_empty()
            || self.other_id.trim().is_empty()
            || self.subject_id == self.other_id
            || !valid_date(&self.occurred_on)
            || self.summary.trim().is_empty()
            || self.summary.len() > 500
            || self.source.trim().is_empty()
            || self.source.len() > 120
            || !(1..=100).contains(&self.salience)
            || self
                .active_until
                .as_deref()
                .is_some_and(|date| !valid_date(date))
            || self
                .active_until
                .as_deref()
                .is_some_and(|date| date < self.occurred_on.as_str())
        {
            return Err("Invalid relationship memory.".into());
        }
        self.impact.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum InteractionKind {
    IntroduceYourself,
    CheckIn,
    PraiseRecentWork,
    OfferEncouragement,
    AskForCreativeInput,
    DiscussColleague,
    ClearTheAir,
}

impl InteractionKind {
    pub const ALL: &'static [Self] = &[
        Self::IntroduceYourself,
        Self::CheckIn,
        Self::PraiseRecentWork,
        Self::OfferEncouragement,
        Self::AskForCreativeInput,
        Self::DiscussColleague,
        Self::ClearTheAir,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::IntroduceYourself => "Introduce yourself",
            Self::CheckIn => "Check in",
            Self::PraiseRecentWork => "Praise recent work",
            Self::OfferEncouragement => "Offer encouragement",
            Self::AskForCreativeInput => "Ask for creative input",
            Self::DiscussColleague => "Discuss a colleague",
            Self::ClearTheAir => "Clear the air",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::IntroduceYourself => "Begin a proper working relationship.",
            Self::CheckIn => "Ask how they are doing without forcing an agenda.",
            Self::PraiseRecentWork => "Recognise a recent performance.",
            Self::OfferEncouragement => "Support them through low morale or confidence.",
            Self::AskForCreativeInput => "Invite their honest view of their presentation.",
            Self::DiscussColleague => "Ask privately how they feel about another wrestler.",
            Self::ClearTheAir => "Address strain between them and management directly.",
        }
    }

    pub fn attention_cost(self) -> u32 {
        if self == Self::ClearTheAir { 2 } else { 1 }
    }

    pub fn cooldown_days(self) -> Option<u64> {
        match self {
            Self::IntroduceYourself => None,
            Self::CheckIn | Self::DiscussColleague => Some(7),
            Self::PraiseRecentWork | Self::OfferEncouragement | Self::AskForCreativeInput => {
                Some(14)
            }
            Self::ClearTheAir => Some(28),
        }
    }

    pub fn requires_target(self) -> bool {
        self == Self::DiscussColleague
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum InteractionTone {
    Warm,
    Open,
    Guarded,
    Defensive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipSignals {
    pub affinity: String,
    pub respect: String,
    pub trust: String,
    pub tension: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipMemoryView {
    pub id: String,
    pub occurred_on: String,
    pub kind: RelationshipMemoryKind,
    pub summary: String,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct PersonalRelationshipView {
    pub other_id: String,
    pub other_name: String,
    pub summary: String,
    pub signals: RelationshipSignals,
    pub memories: Vec<RelationshipMemoryView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ManagementRelationshipView {
    pub summary: String,
    pub signals: RelationshipSignals,
    pub revision: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct InteractionTarget {
    pub worker_id: String,
    pub name: String,
    pub relationship: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct InteractionTargetPage {
    pub rows: Vec<InteractionTarget>,
    pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct InteractionOption {
    pub kind: InteractionKind,
    pub label: String,
    pub description: String,
    pub enabled: bool,
    pub unavailable_reason: Option<String>,
    pub attention_cost: u32,
    pub cooldown_until: Option<String>,
    pub requires_target: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct InteractionOutcome {
    pub request_id: String,
    pub worker_id: String,
    pub kind: InteractionKind,
    pub label: String,
    pub occurred_on: String,
    pub tone: InteractionTone,
    pub response: String,
    pub factors: Vec<String>,
    pub relationship_delta: RelationshipDelta,
    pub morale_delta: i32,
    pub confidence_delta: i32,
    pub effects: Vec<String>,
    pub context_worker_id: Option<String>,
}

impl InteractionOutcome {
    pub fn validate(&self) -> Result<(), String> {
        if !valid_token(&self.request_id, 120)
            || self.worker_id.trim().is_empty()
            || self.label != self.kind.label()
            || !valid_date(&self.occurred_on)
            || self.response.trim().is_empty()
            || self.response.len() > 2000
            || self.factors.is_empty()
            || self.factors.len() > 8
            || self.effects.is_empty()
            || self.effects.len() > 8
            || self.context_worker_id.is_some() != self.kind.requires_target()
            || self.context_worker_id.as_deref() == Some(self.worker_id.as_str())
            || !(-100..=100).contains(&self.morale_delta)
            || !(-100..=100).contains(&self.confidence_delta)
            || self
                .factors
                .iter()
                .chain(&self.effects)
                .any(|value| value.trim().is_empty() || value.len() > 500)
        {
            return Err("Invalid interaction outcome.".into());
        }
        self.relationship_delta.validate()?;
        if self.effects
            != interaction_effects(
                self.relationship_delta,
                self.morale_delta,
                self.confidence_delta,
            )
        {
            return Err("Interaction effects do not match the applied changes.".into());
        }
        Ok(())
    }
}

pub fn interaction_effects(
    relationship: RelationshipDelta,
    morale: i32,
    confidence: i32,
) -> Vec<String> {
    let mut effects = vec![];
    for (value, positive, negative) in [
        (
            relationship.affinity,
            "Management rapport improved.",
            "Management rapport worsened.",
        ),
        (
            relationship.respect,
            "Respect for management improved.",
            "Respect for management fell.",
        ),
        (
            relationship.trust,
            "Trust in management improved.",
            "Trust in management fell.",
        ),
        (morale, "Morale improved.", "Morale fell."),
        (confidence, "Confidence improved.", "Confidence fell."),
    ] {
        if value > 0 {
            effects.push(positive.into());
        } else if value < 0 {
            effects.push(negative.into());
        }
    }
    if relationship.tension < 0 {
        effects.push("Tension eased.".into());
    } else if relationship.tension > 0 {
        effects.push("Tension increased.".into());
    }
    if effects.is_empty() {
        effects.push("No immediate change.".into());
    }
    effects
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipProfile {
    pub rule_version: u32,
    pub management: ManagementRelationshipView,
    pub personal: Vec<PersonalRelationshipView>,
    pub interaction_options: Vec<InteractionOption>,
    pub interaction_history: Vec<InteractionOutcome>,
    pub attention_remaining: u32,
    pub attention_limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InteractionRequest {
    pub save_id: String,
    pub request_id: String,
    pub worker_id: String,
    pub kind: InteractionKind,
    pub context_worker_id: Option<String>,
    pub expected_revision: u32,
}

impl InteractionRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.save_id.trim().is_empty()
            || !valid_token(&self.request_id, 120)
            || self.worker_id.trim().is_empty()
            || self.context_worker_id.as_deref() == Some(self.worker_id.as_str())
        {
            return Err("Invalid interaction request.".into());
        }
        Ok(())
    }
}

pub fn signals(scores: RelationshipScores) -> RelationshipSignals {
    RelationshipSignals {
        affinity: signed_band(scores.affinity, "dislikes", "likes"),
        respect: signed_band(scores.respect, "dismissive", "respectful"),
        trust: signed_band(scores.trust, "distrustful", "trusting"),
        tension: match scores.tension {
            0..=14 => "Calm",
            15..=34 => "Uneasy",
            35..=59 => "Strained",
            60..=79 => "Volatile",
            _ => "Open conflict",
        }
        .into(),
    }
}

pub fn relationship_summary(scores: RelationshipScores) -> String {
    match scores {
        s if s.tension >= 70 || s.affinity <= -65 => "Hostile relationship",
        s if s.trust <= -55 => "Deep distrust",
        s if s.affinity >= 70 && s.trust >= 50 => "Close bond",
        s if s.respect >= 65 && s.affinity >= 20 => "Strong professional bond",
        s if s.affinity >= 40 => "Friendly",
        s if s.respect >= 45 => "Professional respect",
        s if s.tension >= 35 => "Strained",
        s if s.affinity <= -30 => "Personal dislike",
        _ => "Neutral colleague",
    }
    .into()
}

pub fn management_summary(scores: RelationshipScores) -> String {
    match scores {
        s if s.tension >= 70 || s.trust <= -65 => "Open conflict with management",
        s if s.tension >= 35 || s.trust <= -35 => "Strained management relationship",
        s if s.trust >= 65 && s.affinity >= 45 => "Trusted management relationship",
        s if s.affinity >= 35 || s.trust >= 35 => "Positive management relationship",
        s if s.respect >= 35 => "Professional management relationship",
        _ => "New management relationship",
    }
    .into()
}

fn signed_band(value: i32, negative: &str, positive: &str) -> String {
    let strength = match value.unsigned_abs() {
        0..=14 => return "Neutral".into(),
        15..=34 => "Slightly",
        35..=59 => "Clearly",
        60..=79 => "Strongly",
        _ => "Deeply",
    };
    format!("{strength} {}", if value < 0 { negative } else { positive })
}

fn valid_date(value: &str) -> bool {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").is_ok()
}

fn valid_token(value: &str, max: usize) -> bool {
    !value.is_empty()
        && value.len() <= max
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}
