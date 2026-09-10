//! Event-derived distinctions. Source systems supply facts; this module never creates incidents.
use crate::{identity::IdentityVisibility, ratings::Rating100};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use ts_rs::TS;

pub const TRAIT_RULE_VERSION: u32 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum IdentityEventKind {
    MissedMediaBooking,
    ConfidentialityBreach,
    PolicyBreach,
    CampaignCompleted,
    CompanyMilestone,
    CompanyStanding,
    ExternalProject,
    MainstreamRecognition,
    PublicComment,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum EvidenceOutcome {
    Confirmed,
    Positive,
    Unsubstantiated,
    Retracted,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdentityEvent {
    pub id: String,
    pub incident_id: String,
    pub person_id: String,
    pub kind: IdentityEventKind,
    pub occurred_on: String,
    pub recorded_on: String,
    pub company_id: Option<String>,
    pub visibility: IdentityVisibility,
    pub outcome: EvidenceOutcome,
    pub source: String,
    pub supersedes: Option<String>,
    pub recognition: Option<Rating100>,
    pub service_days: Option<u32>,
}
fn date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .filter(|d| d.format("%Y-%m-%d").to_string() == s)
        .ok_or_else(|| "Invalid identity event date.".into())
}
fn id_ok(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 160
}
impl IdentityEvent {
    pub fn validate(&self, person: &str, as_of: &str) -> Result<(), String> {
        let occurred = date(&self.occurred_on)?;
        let recorded = date(&self.recorded_on)?;
        if !id_ok(&self.id)
            || !id_ok(&self.incident_id)
            || self.person_id != person
            || !id_ok(person)
            || self.source.trim().is_empty()
            || self.source.len() > 500
            || occurred > recorded
            || recorded > date(as_of)?
            || self.company_id.as_ref().is_some_and(|s| !id_ok(s))
            || self
                .supersedes
                .as_ref()
                .is_some_and(|s| !id_ok(s) || s == &self.id)
        {
            return Err("Invalid identity evidence, scope or date.".into());
        }
        if let IdentityVisibility::Company(company) = &self.visibility
            && (!id_ok(company) || self.company_id.as_ref() != Some(company))
        {
            return Err("Private evidence must identify its authorised company.".into());
        }
        if matches!(
            self.kind,
            IdentityEventKind::CompanyMilestone
                | IdentityEventKind::CompanyStanding
                | IdentityEventKind::PolicyBreach
        ) && self.company_id.is_none()
        {
            return Err("This event requires a company scope.".into());
        }
        let metrics = match self.kind {
            IdentityEventKind::CompanyStanding => {
                self.recognition.is_some() && self.service_days.is_some()
            }
            IdentityEventKind::MainstreamRecognition => {
                self.recognition.is_some() && self.service_days.is_none()
            }
            _ => self.recognition.is_none() && self.service_days.is_none(),
        };
        if !metrics {
            return Err("Event measurements do not match its kind.".into());
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum ExceptionalTrait {
    MediaLiability,
    WellnessComplianceRisk,
    SponsorFriendly,
    CompanyIcon,
    CrossoverCelebrity,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum TraitStatus {
    Inactive,
    Active,
    Elevated,
    Historical,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct TraitState {
    pub trait_id: ExceptionalTrait,
    pub company_id: Option<String>,
    pub visibility: IdentityVisibility,
    pub status: TraitStatus,
    pub evidence_ids: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct TraitTransition {
    pub date: String,
    pub rule_version: u32,
    pub from: TraitStatus,
    pub state: TraitState,
    pub reason: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TraitLedger {
    pub rule_version: u32,
    pub revision: u32,
    pub evaluated_on: Option<String>,
    pub events: Vec<IdentityEvent>,
    pub states: Vec<TraitState>,
    pub history: Vec<TraitTransition>,
}
impl Default for TraitLedger {
    fn default() -> Self {
        Self {
            rule_version: TRAIT_RULE_VERSION,
            revision: 0,
            evaluated_on: None,
            events: vec![],
            states: vec![],
            history: vec![],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct TraitOverview {
    pub states: Vec<TraitState>,
    pub history: Vec<TraitTransition>,
    pub has_evidence: bool,
}

impl TraitLedger {
    /// Validate a deserialized ledger before its derived state reaches a caller.
    pub fn validate_loaded(&self, person: &str) -> Result<(), String> {
        if self.rule_version != TRAIT_RULE_VERSION || self.revision < self.events.len() as u32 {
            return Err("Invalid persisted trait ledger version or revision.".into());
        }
        let Some(as_of) = self.evaluated_on.as_deref() else {
            return if self.revision == 0
                && self.events.is_empty()
                && self.states.is_empty()
                && self.history.is_empty()
            {
                Ok(())
            } else {
                Err("A populated trait ledger requires an evaluation date.".into())
            };
        };
        date(as_of)?;
        let event_ids: BTreeSet<_> = self.events.iter().map(|event| event.id.as_str()).collect();
        let mut state_keys = BTreeSet::new();
        for state in &self.states {
            if !state_keys.insert((
                state.trait_id,
                state.company_id.as_deref(),
                &state.visibility,
            )) || state
                .evidence_ids
                .iter()
                .any(|id| !event_ids.contains(id.as_str()))
            {
                return Err("Invalid persisted trait state or evidence reference.".into());
            }
        }
        let mut previous_date = None;
        for transition in &self.history {
            let transition_date = date(&transition.date)?;
            if transition.rule_version != TRAIT_RULE_VERSION
                || transition.from == transition.state.status
                || transition.reason.trim().is_empty()
                || transition.reason.len() > 500
                || transition_date > date(as_of)?
                || previous_date.is_some_and(|prior| transition_date < prior)
                || transition
                    .state
                    .evidence_ids
                    .iter()
                    .any(|id| !event_ids.contains(id.as_str()))
            {
                return Err("Invalid persisted trait history.".into());
            }
            previous_date = Some(transition_date);
        }
        let mut evaluated = self.clone();
        let history_len = evaluated.history.len();
        evaluated.evaluate(person, as_of)?;
        if evaluated.states != self.states || evaluated.history.len() != history_len {
            return Err("Persisted trait state does not match its evidence.".into());
        }
        Ok(())
    }

    pub fn view(&self, company: Option<&str>) -> TraitOverview {
        // A later confidential correction must also suppress the original public history.
        let visible_ids: BTreeSet<_> = self
            .events
            .iter()
            .filter(|e| e.visibility.allows(company))
            .map(|e| e.id.as_str())
            .collect();
        let allowed = |s: &TraitState| {
            s.visibility.allows(company)
                && s.evidence_ids
                    .iter()
                    .all(|id| visible_ids.contains(id.as_str()))
        };
        TraitOverview {
            states: self
                .states
                .iter()
                .filter(|s| allowed(s) && s.status != TraitStatus::Inactive)
                .cloned()
                .collect(),
            history: self
                .history
                .iter()
                .rev()
                .filter(|h| allowed(&h.state))
                .take(30)
                .cloned()
                .collect(),
            has_evidence: !visible_ids.is_empty(),
        }
    }
    pub fn record(
        &mut self,
        event: IdentityEvent,
        person: &str,
        as_of: &str,
    ) -> Result<bool, String> {
        event.validate(person, as_of)?;
        if self.rule_version != TRAIT_RULE_VERSION {
            return Err("Unsupported trait rule version.".into());
        }
        if let Some(old) = self.events.iter().find(|e| e.id == event.id) {
            return if *old == event {
                Ok(false)
            } else {
                Err("An event ID cannot be reused for different evidence.".into())
            };
        }
        let last = self
            .events
            .iter()
            .rev()
            .find(|e| e.incident_id == event.incident_id);
        if let Some(previous) = last {
            if event.supersedes.as_deref() != Some(previous.id.as_str()) {
                return Err(
                    "A repeated incident must explicitly correct its latest evidence.".into(),
                );
            }
            if event.person_id != previous.person_id
                || event.company_id != previous.company_id
                || event.kind != previous.kind
                || event.occurred_on != previous.occurred_on
                || event.recorded_on < previous.recorded_on
                || event.visibility != previous.visibility
            {
                return Err(
                    "Corrections must preserve incident identity, date and visibility.".into(),
                );
            }
        } else if event.supersedes.is_some() {
            return Err("Correction target is missing.".into());
        }
        // Work on a clone: validation/evaluation failures must not partially change the ledger.
        let mut next = self.clone();
        next.events.push(event);
        next.evaluate(person, as_of)?;
        next.revision = next
            .revision
            .checked_add(1)
            .ok_or("Trait revision exhausted")?;
        *self = next;
        Ok(true)
    }
    pub fn evaluate(&mut self, person: &str, as_of: &str) -> Result<(), String> {
        if self.rule_version != TRAIT_RULE_VERSION {
            return Err("Unsupported trait rule version.".into());
        }
        let today = date(as_of)?;
        if self
            .evaluated_on
            .as_ref()
            .is_some_and(|d| d.as_str() > as_of)
        {
            return Err("Trait evaluation cannot move backwards.".into());
        }
        let mut effective: BTreeMap<String, &IdentityEvent> = BTreeMap::new();
        let mut ids = BTreeSet::new();
        for e in &self.events {
            e.validate(person, as_of)?;
            if !ids.insert(&e.id) {
                return Err("Duplicate evidence ID.".into());
            }
            if let Some(previous) = effective.get(&e.incident_id) {
                if e.supersedes.as_deref() != Some(previous.id.as_str())
                    || e.kind != previous.kind
                    || e.company_id != previous.company_id
                    || e.visibility != previous.visibility
                    || e.occurred_on != previous.occurred_on
                    || e.recorded_on < previous.recorded_on
                {
                    return Err("Invalid persisted correction chain.".into());
                }
            } else if e.supersedes.is_some() {
                return Err("Correction target is missing.".into());
            }
            effective.insert(e.incident_id.clone(), e);
        }
        let events: Vec<_> = effective
            .values()
            .copied()
            .filter(|e| {
                !matches!(
                    e.outcome,
                    EvidenceOutcome::Retracted | EvidenceOutcome::Unsubstantiated
                )
            })
            .collect();
        let window = |e: &&IdentityEvent, days: i64| {
            today
                .signed_duration_since(date(&e.occurred_on).expect("validated"))
                .num_days()
                < days
        };
        let public = |e: &&IdentityEvent| e.visibility == IdentityVisibility::Public;
        let confirmed = |e: &&IdentityEvent| e.outcome == EvidenceOutcome::Confirmed;
        let mut states = vec![];
        let make = |trait_id, company_id, visibility, status, matching: Vec<&IdentityEvent>| {
            let evidence_ids = matching
                .iter()
                .map(|e| e.id.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            TraitState {
                trait_id,
                company_id,
                visibility,
                status,
                evidence_ids,
            }
        };
        let media: Vec<_> = events
            .iter()
            .copied()
            .filter(public)
            .filter(confirmed)
            .filter(|e| {
                matches!(
                    e.kind,
                    IdentityEventKind::MissedMediaBooking
                        | IdentityEventKind::ConfidentialityBreach
                ) && window(e, 540)
            })
            .collect();
        let risk = |count, active, elevated| {
            if count >= elevated {
                TraitStatus::Elevated
            } else if count >= active {
                TraitStatus::Active
            } else {
                TraitStatus::Inactive
            }
        };
        states.push(make(
            ExceptionalTrait::MediaLiability,
            None,
            IdentityVisibility::Public,
            risk(media.len(), 3, 5),
            media,
        ));
        let campaigns: Vec<_> = events
            .iter()
            .copied()
            .filter(public)
            .filter(|e| {
                e.kind == IdentityEventKind::CampaignCompleted
                    && e.outcome == EvidenceOutcome::Positive
                    && window(e, 730)
            })
            .collect();
        states.push(make(
            ExceptionalTrait::SponsorFriendly,
            None,
            IdentityVisibility::Public,
            if campaigns.len() >= 3 {
                TraitStatus::Active
            } else {
                TraitStatus::Inactive
            },
            campaigns,
        ));
        // Company policy is company-scoped; breaches of unrelated employers' policies never combine.
        let companies: BTreeSet<_> = events
            .iter()
            .filter_map(|e| e.company_id.clone())
            .chain(self.states.iter().filter_map(|s| s.company_id.clone()))
            .collect();
        for company in companies {
            for visibility in [
                IdentityVisibility::Public,
                IdentityVisibility::Company(company.clone()),
            ] {
                let breaches: Vec<_> = events
                    .iter()
                    .copied()
                    .filter(confirmed)
                    .filter(|e| {
                        e.kind == IdentityEventKind::PolicyBreach
                            && e.company_id.as_ref() == Some(&company)
                            && e.visibility == visibility
                            && window(e, 730)
                    })
                    .collect();
                states.push(make(
                    ExceptionalTrait::WellnessComplianceRisk,
                    Some(company.clone()),
                    visibility,
                    risk(breaches.len(), 2, 3),
                    breaches,
                ));
            }
            let milestones: Vec<_> = events
                .iter()
                .copied()
                .filter(public)
                .filter(confirmed)
                .filter(|e| {
                    e.kind == IdentityEventKind::CompanyMilestone
                        && e.company_id.as_ref() == Some(&company)
                })
                .collect();
            let standing = events
                .iter()
                .copied()
                .filter(public)
                .filter(confirmed)
                .filter(|e| {
                    e.kind == IdentityEventKind::CompanyStanding
                        && e.company_id.as_ref() == Some(&company)
                })
                .max_by_key(|e| (&e.occurred_on, &e.id));
            let old = self.states.iter().find(|s| {
                s.trait_id == ExceptionalTrait::CompanyIcon
                    && s.company_id.as_ref() == Some(&company)
                    && s.status == TraitStatus::Historical
            });
            let qualifies = milestones.len() >= 3
                && standing.is_some_and(|e| {
                    e.service_days.unwrap_or(0) >= 1825
                        && e.recognition.is_some_and(|v| v.get() >= 80)
                });
            let mut evidence = milestones;
            if let Some(s) = standing {
                evidence.push(s);
            }
            if qualifies {
                states.push(make(
                    ExceptionalTrait::CompanyIcon,
                    Some(company),
                    IdentityVisibility::Public,
                    TraitStatus::Historical,
                    evidence,
                ));
            } else if let Some(old) = old {
                // An explicit evidence correction can revoke a mistaken distinction; normal departure cannot.
                let corrected = self
                    .events
                    .iter()
                    .filter_map(|e| e.supersedes.as_ref())
                    .any(|id| old.evidence_ids.contains(id));
                if !corrected {
                    states.push(old.clone());
                } else {
                    states.push(TraitState {
                        trait_id: ExceptionalTrait::CompanyIcon,
                        company_id: Some(company),
                        visibility: IdentityVisibility::Public,
                        status: TraitStatus::Inactive,
                        evidence_ids: evidence.iter().map(|e| e.id.clone()).collect(),
                    });
                }
            }
        }
        let projects: Vec<_> = events
            .iter()
            .copied()
            .filter(public)
            .filter(|e| {
                e.kind == IdentityEventKind::ExternalProject
                    && e.outcome == EvidenceOutcome::Positive
                    && window(e, 1095)
            })
            .collect();
        let recognition = events
            .iter()
            .copied()
            .filter(public)
            .filter(confirmed)
            .filter(|e| e.kind == IdentityEventKind::MainstreamRecognition)
            .max_by_key(|e| (&e.occurred_on, &e.id));
        let old = self
            .states
            .iter()
            .find(|s| s.trait_id == ExceptionalTrait::CrossoverCelebrity);
        let threshold = if old.is_some_and(|s| s.status == TraitStatus::Active) {
            60
        } else {
            70
        };
        let status = if projects.len() >= 2
            && recognition.is_some_and(|e| e.recognition.is_some_and(|r| r.get() >= threshold))
        {
            TraitStatus::Active
        } else if old.is_some_and(|s| s.status != TraitStatus::Inactive)
            && !self
                .events
                .iter()
                .filter_map(|e| e.supersedes.as_ref())
                .any(|id| old.is_some_and(|s| s.evidence_ids.contains(id)))
        {
            TraitStatus::Historical
        } else {
            TraitStatus::Inactive
        };
        let mut evidence = projects;
        if let Some(e) = recognition {
            evidence.push(e);
        }
        states.push(TraitState {
            trait_id: ExceptionalTrait::CrossoverCelebrity,
            company_id: None,
            visibility: IdentityVisibility::Public,
            status,
            evidence_ids: if status == TraitStatus::Historical {
                old.map(|s| s.evidence_ids.clone()).unwrap_or_default()
            } else {
                evidence.iter().map(|e| e.id.clone()).collect()
            },
        });
        for state in &states {
            let old = self.states.iter().find(|s| {
                s.trait_id == state.trait_id
                    && s.company_id == state.company_id
                    && s.visibility == state.visibility
            });
            let from = old.map(|s| s.status).unwrap_or(TraitStatus::Inactive);
            if from != state.status {
                let corrected = self
                    .events
                    .last()
                    .is_some_and(|e| e.supersedes.is_some() && e.recorded_on == as_of);
                self.history.push(TraitTransition {
                    date: as_of.into(),
                    rule_version: TRAIT_RULE_VERSION,
                    from,
                    state: state.clone(),
                    reason: format!(
                        "{}: {} eligible evidence item(s); rule version {}.",
                        if corrected {
                            "Evidence correction"
                        } else {
                            "Evidence/window evaluation"
                        },
                        state.evidence_ids.len(),
                        TRAIT_RULE_VERSION
                    ),
                });
            }
        }
        self.states = states;
        self.evaluated_on = Some(as_of.into());
        Ok(())
    }
}
