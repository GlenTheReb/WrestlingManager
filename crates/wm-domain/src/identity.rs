//! Canonical person facts and viewer-safe, deterministic personality descriptions.
use crate::ratings::Rating100;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(tag = "kind", content = "companyId", rename_all = "camelCase")]
pub enum IdentityVisibility {
    Public,
    Company(String),
    Hidden,
}
impl IdentityVisibility {
    pub fn allows(&self, company: Option<&str>) -> bool {
        match self {
            Self::Public => true,
            Self::Company(id) => company == Some(id.as_str()),
            Self::Hidden => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Assessment {
    pub value: Option<Rating100>,
    pub visibility: IdentityVisibility,
    pub source: String,
}
impl Default for Assessment {
    fn default() -> Self {
        Self {
            value: None,
            visibility: IdentityVisibility::Public,
            source: String::new(),
        }
    }
}

macro_rules! catalogue {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
        #[serde(rename_all = "camelCase")]
        pub enum $name { $($variant),+ }
        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
            pub fn label(self) -> &'static str { match self { $(Self::$variant => $label),+ } }
        }
    };
}
catalogue!(PersonalityField { Ambition=>"Ambition", Sociability=>"Sociability", Empathy=>"Empathy", Loyalty=>"Loyalty", Ego=>"Ego", Integrity=>"Integrity", Temperament=>"Temperament", Outspokenness=>"Outspokenness" });
catalogue!(QualityField { Communication=>"Communication", Leadership=>"Leadership", Creativity=>"Creativity", PersonalAdaptability=>"Personal Adaptability", Organisation=>"Organisation", Professionalism=>"Professionalism", WorkEthic=>"Work Ethic", StressManagement=>"Stress Management", Teaching=>"Teaching" });
catalogue!(Motivation { Achievement=>"Achievement", Fame=>"Fame", Money=>"Money", Stability=>"Stability", Belonging=>"Belonging", WrestlingCraft=>"Wrestling Craft", Entertainment=>"Entertainment", CreativeInfluence=>"Creative Influence", Exploration=>"Exploration", Legacy=>"Legacy" });
catalogue!(Hobby { Gaming=>"Gaming", Music=>"Music", Acting=>"Acting", Fashion=>"Fashion", Fitness=>"Fitness", CombatSports=>"Combat sports", Sports=>"Sports", Cars=>"Cars", Travel=>"Travel", Cooking=>"Cooking", Outdoors=>"Outdoors", Reading=>"Reading", Art=>"Art", Collecting=>"Collecting", Charity=>"Charity", Nightlife=>"Nightlife" });
catalogue!(Involvement { Casual=>"Casual", Regular=>"Regular", Passionate=>"Passionate" });
catalogue!(LanguageLevel { Basic=>"Basic", Conversational=>"Conversational", Fluent=>"Fluent" });
catalogue!(BiographyMode { Authored=>"Authored", Organic=>"Organic", Hybrid=>"Hybrid" });

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Motivations {
    pub primary: Motivation,
    pub secondary: Vec<Motivation>,
}
impl Motivations {
    pub fn validate(&self) -> Result<(), String> {
        let mut unique = BTreeSet::from([self.primary]);
        if self.secondary.len() > 2 || self.secondary.iter().any(|m| !unique.insert(*m)) {
            return Err(
                "Motivations must be unique, with at most two secondary priorities.".into(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpokenLanguage {
    pub name: String,
    pub proficiency: Option<LanguageLevel>,
    pub native: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Interest {
    pub hobby: Hobby,
    pub involvement: Involvement,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Biography {
    pub mode: BiographyMode,
    pub authored_text: String,
    pub birthplace: Option<String>,
    pub debut_year: Option<i32>,
    pub trainers: Vec<String>,
    pub previous_occupations: Vec<String>,
}
impl Default for Biography {
    fn default() -> Self {
        Self {
            mode: BiographyMode::Organic,
            authored_text: String::new(),
            birthplace: None,
            debut_year: None,
            trainers: vec![],
            previous_occupations: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersonIdentity {
    pub personality: BTreeMap<PersonalityField, Assessment>,
    pub qualities: BTreeMap<QualityField, Assessment>,
    pub motivations: Option<Motivations>,
    pub languages: Vec<SpokenLanguage>,
    pub hobbies: Vec<Interest>,
    pub biography: Biography,
}
impl Default for PersonIdentity {
    fn default() -> Self {
        Self {
            personality: PersonalityField::ALL
                .iter()
                .map(|p| (*p, Assessment::default()))
                .collect(),
            qualities: QualityField::ALL
                .iter()
                .map(|q| (*q, Assessment::default()))
                .collect(),
            motivations: None,
            languages: vec![],
            hobbies: vec![],
            biography: Biography::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityDescriptor {
    pub field: String,
    pub label: String,
    pub source: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityDescription {
    pub rule_version: u32,
    pub text: String,
    pub partial: bool,
    pub reasons: Vec<PersonalityDescriptor>,
}

impl PersonIdentity {
    pub fn validate(&self) -> Result<(), String> {
        if self.personality.len() != PersonalityField::ALL.len()
            || self.qualities.len() != QualityField::ALL.len()
        {
            return Err("Identity must contain every personality and shared quality field; use unknown where needed.".into());
        }
        for a in self.personality.values().chain(self.qualities.values()) {
            if a.source.len() > 500
                || (a.value.is_some() && a.source.trim().is_empty())
                || matches!(&a.visibility, IdentityVisibility::Company(id) if id.trim().is_empty() || id.len()>80)
            {
                return Err("Invalid assessment provenance or visibility.".into());
            }
        }
        if let Some(m) = &self.motivations {
            m.validate()?;
        }
        let mut languages = BTreeSet::new();
        if self.languages.len() > 32
            || self.languages.iter().any(|l| {
                l.name.trim().is_empty()
                    || l.name.len() > 80
                    || l.name != l.name.trim()
                    || !languages.insert(l.name.to_lowercase())
                    || l.native
                        && matches!(
                            l.proficiency,
                            Some(LanguageLevel::Basic | LanguageLevel::Conversational)
                        )
            })
        {
            return Err("Invalid or duplicate spoken language.".into());
        }
        let mut hobbies = BTreeSet::new();
        if self.hobbies.len() > 5 || self.hobbies.iter().any(|i| !hobbies.insert(i.hobby)) {
            return Err("At most five unique interests are allowed.".into());
        }
        let b = &self.biography;
        if b.authored_text.len() > 8000
            || matches!(b.mode, BiographyMode::Authored | BiographyMode::Hybrid)
                && b.authored_text.trim().is_empty()
            || b.debut_year.is_some_and(|y| !(1800..=9999).contains(&y))
            || b.birthplace
                .as_ref()
                .is_some_and(|s| s.trim().is_empty() || s.len() > 160)
            || b.trainers.len() > 12
            || b.previous_occupations.len() > 12
            || b.trainers
                .iter()
                .chain(&b.previous_occupations)
                .any(|s| s.trim().is_empty() || s.len() > 160)
        {
            return Err("Invalid biography facts.".into());
        }
        Ok(())
    }

    /// Strip secret values AND provenance before a DTO leaves the trusted Rust boundary.
    pub fn for_viewer(&self, company: Option<&str>) -> Self {
        let mut projected = self.clone();
        for a in projected
            .personality
            .values_mut()
            .chain(projected.qualities.values_mut())
        {
            if !a.visibility.allows(company) {
                *a = Assessment::default();
            }
        }
        projected
    }

    pub fn describe(&self, company: Option<&str>) -> PersonalityDescription {
        let visible = self.for_viewer(company);
        let mut inputs = vec![
            (
                "professionalism",
                "Dependable",
                "Inconsistent with obligations",
                visible.qualities.get(&QualityField::Professionalism),
            ),
            (
                "workEthic",
                "Hard-working",
                "Low training effort",
                visible.qualities.get(&QualityField::WorkEthic),
            ),
        ];
        for (p, id, high, low) in [
            (
                PersonalityField::Ambition,
                "ambition",
                "Ambitious",
                "Content with current position",
            ),
            (
                PersonalityField::Sociability,
                "sociability",
                "Outgoing",
                "Reserved",
            ),
            (
                PersonalityField::Empathy,
                "empathy",
                "Considerate",
                "Self-focused",
            ),
            (PersonalityField::Loyalty, "loyalty", "Loyal", "Independent"),
            (
                PersonalityField::Ego,
                "ego",
                "Status-conscious",
                "Unassuming",
            ),
            (
                PersonalityField::Integrity,
                "integrity",
                "Principled",
                "Expedient",
            ),
            (
                PersonalityField::Temperament,
                "temperament",
                "Even-tempered",
                "Quick-tempered",
            ),
            (
                PersonalityField::Outspokenness,
                "outspokenness",
                "Outspoken",
                "Guarded",
            ),
        ] {
            inputs.push((id, high, low, visible.personality.get(&p)));
        }
        let partial = inputs
            .iter()
            .any(|(_, _, _, a)| a.and_then(|a| a.value).is_none());
        let mut candidates = vec![];
        for (index, (field, high, low, a)) in inputs.iter().enumerate() {
            if let Some(a) = a
                && let Some(value) = a.value
            {
                let v = value.get();
                if v <= 30 || v >= 70 {
                    candidates.push((
                        (v - 50).abs(),
                        index,
                        PersonalityDescriptor {
                            field: (*field).into(),
                            label: if v >= 70 { *high } else { *low }.into(),
                            source: a.source.clone(),
                        },
                    ));
                }
            }
        }
        candidates.sort_by_key(|(distance, index, _)| (std::cmp::Reverse(*distance), *index));
        let reasons: Vec<_> = candidates.into_iter().take(2).map(|(_, _, d)| d).collect();
        let text = if reasons.is_empty() {
            if partial {
                "Personality not yet established"
            } else {
                "No pronounced tendencies"
            }
            .into()
        } else {
            reasons
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    if i == 0 {
                        d.label.clone()
                    } else {
                        d.label.to_lowercase()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        PersonalityDescription {
            rule_version: 1,
            text,
            partial,
            reasons,
        }
    }

    pub fn motivation_text(&self) -> String {
        self.motivations
            .as_ref()
            .map(|m| {
                std::iter::once(m.primary.label())
                    .chain(m.secondary.iter().map(|m| m.label()))
                    .collect::<Vec<_>>()
                    .join(" · ")
            })
            .unwrap_or_else(|| "Motivations not yet known".into())
    }

    pub fn biography_text(
        &self,
        name: &str,
        nationality: &str,
        school: &str,
        background: &str,
        matches: i32,
    ) -> String {
        let b = &self.biography;
        if b.mode == BiographyMode::Authored {
            return b.authored_text.clone();
        }
        let mut text = vec![];
        if b.mode == BiographyMode::Hybrid {
            text.push(b.authored_text.clone());
        }
        if !nationality.is_empty() {
            text.push(format!("{name} is a wrestler from {nationality}."));
        }
        if let Some(place) = &b.birthplace {
            text.push(format!("Birthplace: {place}."));
        }
        if !school.is_empty() {
            text.push(format!("Trained at {school}."));
        }
        if let Some(year) = b.debut_year {
            text.push(format!("Entered wrestling in {year}."));
        }
        if !b.trainers.is_empty() {
            text.push(format!("Trainers: {}.", b.trainers.join(", ")));
        }
        if !b.previous_occupations.is_empty() {
            text.push(format!(
                "Previous occupations: {}.",
                b.previous_occupations.join(", ")
            ));
        }
        if !background.is_empty() {
            text.push(background.into());
        }
        if matches > 0 {
            text.push(format!("{matches} matches recorded in this career."));
        }
        text.join(" ")
    }
}
