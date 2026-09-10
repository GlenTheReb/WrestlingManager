use crate::ratings::{WrestlerAttributes, WrestlingStyleProfile, WrestlingSummary};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub fatigue: i32,
    pub confidence: i32,
    pub morale: i32,
    pub momentum: i32,
    pub popularity: i32,
    pub wear: i32,
    pub injury_days: i32,
    pub development: i32,
    pub matches: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub id: String,
    pub name: String,
    pub style: String,
    pub difficulty: i32,
    pub risk: i32,
    pub stamina_cost: i32,
    pub min_strength: i32,
    pub proficiency: i32,
    pub signature: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub nationality: String,
    pub language: String,
    pub school: String,
    pub background: String,
    pub personality: String,
    pub ambition: String,
    pub style: String,
    pub weight_kg: i32,
    pub attributes: WrestlerAttributes,
    pub wrestling_style: WrestlingStyleProfile,
    pub condition: Condition,
    pub moves: Vec<Move>,
    pub appearance_fee: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct RosterRow {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub style: String,
    pub archetype: String,
    pub overall: i32,
    pub groups: crate::ratings::GroupScores,
    pub condition: Condition,
}
impl From<&Worker> for RosterRow {
    fn from(worker: &Worker) -> Self {
        let summary = worker.wrestling_style.summary(&worker.attributes);
        Self {
            id: worker.id.clone(),
            name: worker.name.clone(),
            age: worker.age,
            style: worker.style.clone(),
            archetype: summary.archetype,
            overall: summary.overall.get(),
            groups: summary.groups,
            condition: worker.condition.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct RosterPage {
    pub rows: Vec<RosterRow>,
    pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct RoadAgent {
    pub id: i32,
    pub name: String,
    pub psychology: i32,
    pub communication: i32,
    pub experience: i32,
    pub safety: i32,
    pub philosophy: String,
    pub knowledge: i32,
    pub trust: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum BeatKind {
    Move,
    Control,
    FalseFinish,
    Interference,
    Weapon,
    RefBump,
    Callback,
    InjurySell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct PlannedBeat {
    pub at_second: u32,
    pub actor_id: String,
    pub kind: BeatKind,
    pub move_id: Option<String>,
    pub duration_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Finish {
    Pinfall,
    Submission,
    CountOut,
    Disqualification,
    Draw,
    NoContest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MatchPlan {
    pub match_type: String,
    pub worker_a: String,
    pub worker_b: String,
    pub winner_id: Option<String>,
    pub finish: Finish,
    pub clean_finish: bool,
    pub duration_seconds: u32,
    pub style: String,
    pub pace: i32,
    pub risk: i32,
    pub freedom: i32,
    pub purpose: String,
    pub protected_worker_id: Option<String>,
    pub agent_id: i32,
    pub beats: Vec<PlannedBeat>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct AnglePlan {
    pub participants: Vec<String>,
    pub purpose: String,
    pub duration_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", content = "plan", rename_all = "camelCase")]
pub enum SegmentPlan {
    Match(MatchPlan),
    Angle(AnglePlan),
}
impl SegmentPlan {
    pub fn duration(&self) -> u32 {
        match self {
            Self::Match(p) => p.duration_seconds,
            Self::Angle(p) => p.duration_seconds,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub id: i32,
    pub title: String,
    pub content: SegmentPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ShowCard {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub status: String,
    pub revision: i32,
    pub segments: Vec<Segment>,
    pub capacity_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveSegmentRequest {
    pub save_id: String,
    pub show_id: i32,
    pub revision: i32,
    pub segment_id: Option<i32>,
    pub content: SegmentPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Cohort {
    pub name: String,
    pub preference: String,
    pub energy: i32,
    pub trust: i32,
    pub fatigue: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CrowdState {
    pub energy: i32,
    pub fatigue: i32,
    pub expectation: i32,
    pub trust: i32,
    pub interference_count: i32,
    pub peak: i32,
    pub cohorts: Vec<Cohort>,
    pub chant: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    pub execution: i32,
    pub psychology: i32,
    pub engagement: i32,
    pub safety: i32,
    pub story: i32,
    pub protection: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SimEvent {
    pub sequence: u32,
    pub show_second: u32,
    pub segment_id: i32,
    pub match_second: u32,
    pub kind: String,
    pub text: String,
    pub actor_id: Option<String>,
    pub move_id: Option<String>,
    pub importance: i32,
    pub crowd_energy: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerChange {
    pub worker_id: String,
    pub name: String,
    pub fatigue: i32,
    pub confidence: i32,
    pub morale: i32,
    pub momentum: i32,
    pub popularity: i32,
    pub development: i32,
    pub wear: i32,
    pub injury_days: i32,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SegmentReport {
    pub segment_id: i32,
    pub title: String,
    pub winner_id: Option<String>,
    pub result: String,
    pub duration_seconds: u32,
    pub performance: Performance,
    pub reasons: Vec<String>,
    pub changes: Vec<WorkerChange>,
    pub chemistry_change: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MediaPost {
    pub id: i32,
    pub author: String,
    pub text: String,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ShowReport {
    pub show_id: i32,
    pub name: String,
    pub date: String,
    pub segments: Vec<SegmentReport>,
    pub crowd: CrowdState,
    pub attendance: i32,
    pub revenue_pence: i32,
    pub costs_pence: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct MatchView {
    pub worker_a: String,
    pub worker_b: String,
    pub name_a: String,
    pub name_b: String,
    pub stamina_a: i32,
    pub stamina_b: i32,
    pub second: u32,
    pub duration_seconds: u32,
    pub phase: String,
    pub pace: i32,
    pub risk: i32,
    pub control: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct LiveView {
    pub show_id: i32,
    pub tick: u32,
    pub segment_index: u32,
    pub complete: bool,
    pub segment_title: String,
    pub current_match: Option<MatchView>,
    pub crowd: CrowdState,
    pub events: Vec<SimEvent>,
    pub pending_instructions: Vec<String>,
    pub report: Option<ShowReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum InstructionKind {
    SlowDown,
    RaisePace,
    Protect,
    GoHome,
    Extend,
    ChangeFinish,
    AbandonSpot,
    TakeRisks,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct LiveInstruction {
    pub kind: InstructionKind,
    pub worker_id: Option<String>,
    pub finish: Option<Finish>,
    pub beat_index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceRequest {
    pub save_id: String,
    pub show_id: i32,
    pub expected_tick: u32,
    pub seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct AgentAdvice {
    pub plan: MatchPlan,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CareerOffice {
    pub promotion: crate::PromotionOverview,
    pub show: ShowCard,
    pub agents: Vec<RoadAgent>,
    pub roster_count: u32,
    pub media: Vec<MediaPost>,
    pub recent_shows: Vec<ShowCard>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkerProfile {
    pub worker: Worker,
    pub wrestling: WrestlingSummary,
    pub history: Vec<SegmentReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct NewsItem {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub body: String,
    pub date: String,
    pub show_id: Option<i32>,
    pub worker_id: Option<String>,
    pub read: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct NewsPage {
    pub items: Vec<NewsItem>,
    pub total: u32,
    pub unread: u32,
}
