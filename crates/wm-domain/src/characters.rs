use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum AlignmentIntent {
    Face,
    Heel,
    Tweener,
    Unaligned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum AudienceResponse {
    Cheered,
    Booed,
    Mixed,
    Indifferent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum PerceivedRole {
    Face,
    Heel,
    Mixed,
    Unclear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ReactionIntensity {
    Mild,
    Moderate,
    Strong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum CharacterAcceptance {
    Embraced,
    Accepted,
    Uncertain,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum IntentMatch {
    Matched,
    Mismatched,
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum IdentityKnowledge {
    Private,
    Rumoured,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum CharacterStatus {
    Planned,
    Active,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ChangeStatus {
    Proposed,
    Negotiating,
    Accepted,
    Refused,
    Ready,
    Launched,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct GimmickBrief {
    pub name: String,
    pub description: String,
    pub core_fantasy: String,
    pub tags: Vec<String>,
    pub tone: String,
    pub promo_voice: String,
    pub presentation_intensity: String,
    pub entrance_and_match_behavior: String,
    pub attire_and_mask: String,
    pub catchphrases_and_gestures: String,
    pub traits_to_emphasize: Vec<String>,
}

impl Default for GimmickBrief {
    fn default() -> Self {
        Self {
            name: "Authentic competitor".into(),
            description: "A grounded presentation built around the performer's existing strengths."
                .into(),
            core_fantasy: "Credible professional wrestler".into(),
            tags: vec!["authentic".into(), "competitive".into()],
            tone: "Serious".into(),
            promo_voice: "Natural".into(),
            presentation_intensity: "Balanced".into(),
            entrance_and_match_behavior: "Focused and competitive".into(),
            attire_and_mask: "Standard ring attire".into(),
            catchphrases_and_gestures: String::new(),
            traits_to_emphasize: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterIdentity {
    pub id: String,
    pub revision: i32,
    pub ring_name: String,
    pub alignment_intent: AlignmentIntent,
    pub status: CharacterStatus,
    pub masked: bool,
    pub concealed: bool,
    pub gimmick: GimmickBrief,
    pub company_id: String,
    pub brand: Option<String>,
    pub started_on: String,
    pub ended_on: Option<String>,
    pub aliases: Vec<String>,
    pub identity_knowledge: IdentityKnowledge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct AudienceResponseEvidence {
    pub date: String,
    pub perceived_role: PerceivedRole,
    pub response: AudienceResponse,
    pub intensity: ReactionIntensity,
    pub acceptance: CharacterAcceptance,
    pub intent_match: IntentMatch,
    pub context: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterChange {
    pub id: i64,
    pub request_id: String,
    pub proposed_ring_name: String,
    pub proposed_alignment: AlignmentIntent,
    pub proposed_masked: bool,
    pub proposed_concealed: bool,
    pub proposed_gimmick: GimmickBrief,
    pub status: ChangeStatus,
    pub revision: i32,
    pub proposed_on: String,
    pub intended_launch_on: Option<String>,
    pub launched_on: Option<String>,
    pub worker_response: String,
    pub readiness: String,
    pub risk: String,
    pub advice: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterProfile {
    pub legal_name: Option<String>,
    pub active: CharacterIdentity,
    pub history: Vec<CharacterIdentity>,
    pub audience_responses: Vec<AudienceResponseEvidence>,
    pub pending_change: Option<CharacterChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProposeCharacterChangeRequest {
    pub save_id: String,
    pub worker_id: String,
    pub request_id: String,
    pub expected_revision: i32,
    pub ring_name: String,
    pub alignment_intent: AlignmentIntent,
    pub masked: bool,
    pub concealed: bool,
    pub intended_launch_on: Option<String>,
    pub gimmick: GimmickBrief,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterActionRequest {
    pub save_id: String,
    pub worker_id: String,
    pub request_id: String,
    pub change_id: Option<i64>,
    pub expected_revision: i32,
}

pub fn validate_public_name(value: &str) -> Result<String, &'static str> {
    let value = value.trim();
    let characters = value.chars().count();
    if !(2..=60).contains(&characters) || value.chars().any(char::is_control) {
        return Err("Ring names must contain 2–60 visible characters.");
    }
    Ok(value.to_owned())
}

pub fn validate_gimmick(brief: &GimmickBrief) -> Result<(), &'static str> {
    fn invalid_text(value: &str, maximum: usize, required: bool) -> bool {
        let trimmed = value.trim();
        (required && trimmed.is_empty())
            || value.chars().count() > maximum
            || value
                .chars()
                .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    }

    if invalid_text(&brief.name, 80, true) {
        return Err("Give the gimmick a concise name.");
    }
    if invalid_text(&brief.description, 600, false)
        || invalid_text(&brief.core_fantasy, 160, true)
        || invalid_text(&brief.tone, 80, false)
        || invalid_text(&brief.promo_voice, 120, false)
        || invalid_text(&brief.presentation_intensity, 40, false)
        || invalid_text(&brief.entrance_and_match_behavior, 240, false)
        || invalid_text(&brief.attire_and_mask, 180, false)
        || invalid_text(&brief.catchphrases_and_gestures, 240, false)
        || brief.tags.len() > 12
        || brief.traits_to_emphasize.len() > 8
        || brief.tags.iter().any(|value| invalid_text(value, 40, true))
        || brief
            .traits_to_emphasize
            .iter()
            .any(|value| invalid_text(value, 60, true))
    {
        return Err("The creative brief exceeds its safe limits.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_names_count_visible_characters_not_utf8_bytes() {
        assert!(validate_public_name("A").is_err());
        assert!(validate_public_name("🔥").is_err());
        assert_eq!(validate_public_name("  Rey  ").unwrap(), "Rey");
    }

    #[test]
    fn gimmick_validation_bounds_every_player_authored_field() {
        let brief = GimmickBrief {
            promo_voice: "x".repeat(121),
            ..GimmickBrief::default()
        };
        assert!(validate_gimmick(&brief).is_err());

        let brief = GimmickBrief {
            tags: vec![String::new()],
            ..GimmickBrief::default()
        };
        assert!(validate_gimmick(&brief).is_err());
    }
}
