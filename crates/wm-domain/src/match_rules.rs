//! Match-local identities and booked results, independent of runtime capabilities.
//!
//! A side is an alliance for this match, not a permanent team/stable. Slot IDs remain
//! stable when workers are replaced or the booking UI changes display order.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

use crate::game::{Finish, MatchPlan};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantSlot {
    pub id: String,
    pub worker_id: String,
    pub side_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MatchSide {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ParticipationRule {
    AllActive,
    Tag,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum VictoryRule {
    OneFall,
    Elimination,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MatchRules {
    pub participation: ParticipationRule,
    pub victory: VictoryRule,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum DecisionMethod {
    Pinfall,
    Submission,
    CountOut,
    Disqualification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(
    tag = "outcome",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum BookedResult {
    Decision {
        winning_side_id: String,
        method: DecisionMethod,
        deciding_slot_id: Option<String>,
        defeated_slot_id: Option<String>,
    },
    // Empty struct variants make Serde enforce unknown-field rejection here too.
    Draw {},
    NoContest {},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MatchDefinition {
    pub slots: Vec<ParticipantSlot>,
    pub sides: Vec<MatchSide>,
    pub rules: MatchRules,
    pub result: BookedResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchRuleError {
    #[error("A match needs at least two populated opposing sides.")]
    OpposingSides,
    #[error(
        "Side, slot and worker identities must be nonempty, unique and have no surrounding whitespace."
    )]
    Identity,
    #[error("Every participant slot must belong to a declared side.")]
    Membership,
    #[error("A tag match needs at least one side with multiple wrestlers.")]
    TagPartners,
    #[error("The booked winning side must participate in the match.")]
    WinningSide,
    #[error("The deciding wrestler must belong to the winning side.")]
    DecidingSlot,
    #[error("The defeated wrestler must belong to an opposing side.")]
    DefeatedSlot,
    #[error("Pinfall and submission results must identify the deciding and defeated wrestlers.")]
    FallParticipants,
    #[error("The booked winner must participate; draws and no contests have no winner.")]
    LegacyWinner,
    #[error("This engine supports singles and no-disqualification singles.")]
    LegacyMatchType,
}

fn valid_identity(id: &str) -> bool {
    !id.is_empty() && id.trim() == id
}

impl MatchDefinition {
    pub fn slot(&self, id: &str) -> Option<&ParticipantSlot> {
        self.slots.iter().find(|slot| slot.id == id)
    }

    pub fn winning_side_id(&self) -> Option<&str> {
        match &self.result {
            BookedResult::Decision {
                winning_side_id, ..
            } => Some(winning_side_id),
            BookedResult::Draw {} | BookedResult::NoContest {} => None,
        }
    }

    pub fn winning_worker_ids(&self) -> impl Iterator<Item = &str> {
        self.slots.iter().filter_map(|slot| {
            (Some(slot.side_id.as_str()) == self.winning_side_id())
                .then_some(slot.worker_id.as_str())
        })
    }

    /// Validate booking structure only. This does not promise runtime support or
    /// check medical availability, legal tagging state or elimination history.
    pub fn validate(&self) -> Result<(), MatchRuleError> {
        let mut sides = BTreeSet::new();
        for side in &self.sides {
            if !valid_identity(&side.id) || !sides.insert(side.id.as_str()) {
                return Err(MatchRuleError::Identity);
            }
        }
        if sides.len() < 2 {
            return Err(MatchRuleError::OpposingSides);
        }
        let mut slots = BTreeSet::new();
        let mut workers = BTreeSet::new();
        let mut populated = BTreeSet::new();
        for slot in &self.slots {
            if !valid_identity(&slot.id)
                || !valid_identity(&slot.worker_id)
                || !slots.insert(slot.id.as_str())
                || !workers.insert(slot.worker_id.as_str())
            {
                return Err(MatchRuleError::Identity);
            }
            if !sides.contains(slot.side_id.as_str()) {
                return Err(MatchRuleError::Membership);
            }
            populated.insert(slot.side_id.as_str());
        }
        if populated != sides {
            return Err(MatchRuleError::OpposingSides);
        }
        if self.rules.participation == ParticipationRule::Tag && slots.len() == sides.len() {
            return Err(MatchRuleError::TagPartners);
        }
        if let BookedResult::Decision {
            winning_side_id,
            method,
            deciding_slot_id,
            defeated_slot_id,
        } = &self.result
        {
            if !sides.contains(winning_side_id.as_str()) {
                return Err(MatchRuleError::WinningSide);
            }
            if matches!(method, DecisionMethod::Pinfall | DecisionMethod::Submission)
                && (deciding_slot_id.is_none() || defeated_slot_id.is_none())
            {
                return Err(MatchRuleError::FallParticipants);
            }
            if deciding_slot_id.as_ref().is_some_and(|id| {
                self.slot(id)
                    .is_none_or(|slot| &slot.side_id != winning_side_id)
            }) {
                return Err(MatchRuleError::DecidingSlot);
            }
            if defeated_slot_id.as_ref().is_some_and(|id| {
                self.slot(id)
                    .is_none_or(|slot| &slot.side_id == winning_side_id)
            }) {
                return Err(MatchRuleError::DefeatedSlot);
            }
        }
        Ok(())
    }
}

impl Finish {
    pub fn decision_method(&self) -> Option<DecisionMethod> {
        match self {
            Self::Pinfall => Some(DecisionMethod::Pinfall),
            Self::Submission => Some(DecisionMethod::Submission),
            Self::CountOut => Some(DecisionMethod::CountOut),
            Self::Disqualification => Some(DecisionMethod::Disqualification),
            Self::Draw | Self::NoContest => None,
        }
    }
}

impl TryFrom<&MatchPlan> for MatchDefinition {
    type Error = MatchRuleError;

    /// Adapt schema-3 singles without changing their saved JSON or inventing team
    /// records. Never use the display label to infer an unsupported future format.
    fn try_from(plan: &MatchPlan) -> Result<Self, Self::Error> {
        if !["Singles", "No disqualification"].contains(&plan.match_type.as_str()) {
            return Err(MatchRuleError::LegacyMatchType);
        }
        let result = match (plan.finish.decision_method(), &plan.winner_id) {
            (Some(method), Some(winner)) => {
                let (winning, defeated) = if winner == &plan.worker_a {
                    ("a", "b")
                } else if winner == &plan.worker_b {
                    ("b", "a")
                } else {
                    return Err(MatchRuleError::LegacyWinner);
                };
                BookedResult::Decision {
                    winning_side_id: format!("side-{winning}"),
                    method,
                    deciding_slot_id: Some(format!("slot-{winning}")),
                    defeated_slot_id: Some(format!("slot-{defeated}")),
                }
            }
            (None, None) if plan.finish == Finish::Draw => BookedResult::Draw {},
            (None, None) => BookedResult::NoContest {},
            _ => return Err(MatchRuleError::LegacyWinner),
        };
        let definition = Self {
            slots: [("a", &plan.worker_a), ("b", &plan.worker_b)]
                .into_iter()
                .map(|(label, worker)| ParticipantSlot {
                    id: format!("slot-{label}"),
                    worker_id: worker.clone(),
                    side_id: format!("side-{label}"),
                })
                .collect(),
            sides: ["side-a", "side-b"]
                .into_iter()
                .map(|id| MatchSide { id: id.into() })
                .collect(),
            rules: MatchRules {
                participation: ParticipationRule::AllActive,
                victory: VictoryRule::OneFall,
            },
            result,
        };
        definition.validate()?;
        Ok(definition)
    }
}
