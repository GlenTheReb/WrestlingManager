use crate::game::RosterRow;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum WorkerSortKey {
    Relevance,
    Name,
    Age,
    Overall,
    Movement,
    Physicality,
    Ringcraft,
    Psychology,
    Fundamentals,
    Entertainment,
    Fatigue,
    Morale,
    Momentum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSort {
    pub key: WorkerSortKey,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct NumberRange {
    pub minimum: Option<i32>,
    pub maximum: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum WorkerAvailability {
    Any,
    Cleared,
    Injured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum BlacklistMode {
    Include,
    Exclude,
    Only,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct WorkerSearchFilters {
    pub age: NumberRange,
    pub overall: NumberRange,
    pub movement: NumberRange,
    pub physicality: NumberRange,
    pub ringcraft: NumberRange,
    pub psychology: NumberRange,
    pub fundamentals: NumberRange,
    pub entertainment: NumberRange,
    pub nationalities: Vec<String>,
    pub excluded_nationalities: Vec<String>,
    pub languages: Vec<String>,
    pub excluded_languages: Vec<String>,
    pub schools: Vec<String>,
    pub excluded_schools: Vec<String>,
    pub archetypes: Vec<String>,
    pub excluded_archetypes: Vec<String>,
    pub primary_disciplines: Vec<String>,
    pub excluded_primary_disciplines: Vec<String>,
    pub availability: WorkerAvailability,
    pub shortlist_id: Option<i32>,
    pub blacklist: BlacklistMode,
}

impl Default for WorkerSearchFilters {
    fn default() -> Self {
        Self {
            age: NumberRange::default(),
            overall: NumberRange::default(),
            movement: NumberRange::default(),
            physicality: NumberRange::default(),
            ringcraft: NumberRange::default(),
            psychology: NumberRange::default(),
            fundamentals: NumberRange::default(),
            entertainment: NumberRange::default(),
            nationalities: Vec::new(),
            excluded_nationalities: Vec::new(),
            languages: Vec::new(),
            excluded_languages: Vec::new(),
            schools: Vec::new(),
            excluded_schools: Vec::new(),
            archetypes: Vec::new(),
            excluded_archetypes: Vec::new(),
            primary_disciplines: Vec::new(),
            excluded_primary_disciplines: Vec::new(),
            availability: WorkerAvailability::Any,
            shortlist_id: None,
            blacklist: BlacklistMode::Include,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSearchRequest {
    pub text: String,
    pub filters: WorkerSearchFilters,
    pub sort: Vec<WorkerSort>,
    pub offset: usize,
    pub limit: usize,
}

impl Default for WorkerSearchRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            filters: WorkerSearchFilters::default(),
            sort: vec![WorkerSort {
                key: WorkerSortKey::Name,
                direction: SortDirection::Ascending,
            }],
            offset: 0,
            limit: 50,
        }
    }
}

impl WorkerSearchRequest {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.text.chars().count() > 80 || self.limit == 0 || self.limit > 100 {
            return Err("Search text is limited to 80 characters and pages to 100 workers.");
        }
        if self.sort.is_empty() || self.sort.len() > 2 {
            return Err("Choose one or two sort fields.");
        }
        let unique = self
            .sort
            .iter()
            .map(|sort| format!("{:?}", sort.key))
            .collect::<BTreeSet<_>>();
        if unique.len() != self.sort.len() {
            return Err("Each sort field can only be used once.");
        }
        validate_range(&self.filters.age, 16, 120)?;
        for range in [
            &self.filters.overall,
            &self.filters.movement,
            &self.filters.physicality,
            &self.filters.ringcraft,
            &self.filters.psychology,
            &self.filters.fundamentals,
            &self.filters.entertainment,
        ] {
            validate_range(range, 0, 100)?;
        }
        for values in [
            &self.filters.nationalities,
            &self.filters.excluded_nationalities,
            &self.filters.languages,
            &self.filters.excluded_languages,
            &self.filters.schools,
            &self.filters.excluded_schools,
            &self.filters.archetypes,
            &self.filters.excluded_archetypes,
            &self.filters.primary_disciplines,
            &self.filters.excluded_primary_disciplines,
        ] {
            if values.len() > 32
                || values
                    .iter()
                    .any(|value| value.trim().is_empty() || value.chars().count() > 64)
            {
                return Err("A filter can contain at most 32 valid values.");
            }
        }
        for (included, excluded) in [
            (
                &self.filters.nationalities,
                &self.filters.excluded_nationalities,
            ),
            (&self.filters.languages, &self.filters.excluded_languages),
            (&self.filters.schools, &self.filters.excluded_schools),
            (&self.filters.archetypes, &self.filters.excluded_archetypes),
            (
                &self.filters.primary_disciplines,
                &self.filters.excluded_primary_disciplines,
            ),
        ] {
            if included.iter().any(|value| {
                excluded
                    .iter()
                    .any(|candidate| candidate.eq_ignore_ascii_case(value))
            }) {
                return Err("A value cannot be both included and excluded.");
            }
        }
        if self.filters.shortlist_id.is_some_and(|id| id <= 0) {
            return Err("The selected shortlist is invalid.");
        }
        Ok(())
    }
}

fn validate_range(range: &NumberRange, floor: i32, ceiling: i32) -> Result<(), &'static str> {
    if range
        .minimum
        .is_some_and(|value| value < floor || value > ceiling)
        || range
            .maximum
            .is_some_and(|value| value < floor || value > ceiling)
        || matches!((range.minimum, range.maximum), (Some(min), Some(max)) if min > max)
    {
        return Err("A filter range is invalid.");
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSearchHit {
    pub worker: RosterRow,
    pub match_reason: Option<String>,
    pub shortlist_ids: Vec<i32>,
    pub blacklisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSearchPage {
    pub rows: Vec<WorkerSearchHit>,
    pub total: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerFilterOptions {
    pub nationalities: Vec<String>,
    pub languages: Vec<String>,
    pub schools: Vec<String>,
    pub archetypes: Vec<String>,
    pub primary_disciplines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SavedWorkerView {
    pub id: i32,
    pub name: String,
    pub request: WorkerSearchRequest,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerShortlist {
    pub id: i32,
    pub name: String,
    pub member_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDiscoveryLists {
    pub saved_views: Vec<SavedWorkerView>,
    pub shortlists: Vec<WorkerShortlist>,
    pub blacklist_count: usize,
}
