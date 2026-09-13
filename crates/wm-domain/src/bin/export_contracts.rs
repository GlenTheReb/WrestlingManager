use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use ts_rs::{Config, TS};
use wm_domain::characters::*;
use wm_domain::discovery::*;
use wm_domain::game::*;
use wm_domain::identity::*;
use wm_domain::match_rules::*;
use wm_domain::ratings::*;
use wm_domain::relationships::*;
use wm_domain::traits::*;
use wm_domain::{CreateGameRequest, IpcError, PromotionOverview, SaveSummary};

fn generated_contracts() -> String {
    let config = Config::default();
    let declarations = [
        CreateGameRequest::decl(&config),
        PromotionOverview::decl(&config),
        SaveSummary::decl(&config),
        IpcError::decl(&config),
        AlignmentIntent::decl(&config),
        AudienceResponse::decl(&config),
        PerceivedRole::decl(&config),
        ReactionIntensity::decl(&config),
        CharacterAcceptance::decl(&config),
        IntentMatch::decl(&config),
        IdentityKnowledge::decl(&config),
        CharacterStatus::decl(&config),
        ChangeStatus::decl(&config),
        GimmickBrief::decl(&config),
        CharacterIdentity::decl(&config),
        AudienceResponseEvidence::decl(&config),
        CharacterChange::decl(&config),
        CharacterProfile::decl(&config),
        ProposeCharacterChangeRequest::decl(&config),
        CharacterActionRequest::decl(&config),
        SortDirection::decl(&config),
        WorkerSortKey::decl(&config),
        WorkerSort::decl(&config),
        NumberRange::decl(&config),
        WorkerAvailability::decl(&config),
        BlacklistMode::decl(&config),
        WorkerSearchFilters::decl(&config),
        WorkerSearchRequest::decl(&config),
        WorkerSearchHit::decl(&config),
        WorkerSearchPage::decl(&config),
        WorkerFilterOptions::decl(&config),
        SavedWorkerView::decl(&config),
        WorkerShortlist::decl(&config),
        WorkerDiscoveryLists::decl(&config),
        Rating100::decl(&config),
        IdentityVisibility::decl(&config),
        Assessment::decl(&config),
        PersonalityField::decl(&config),
        QualityField::decl(&config),
        Motivation::decl(&config),
        Motivations::decl(&config),
        Hobby::decl(&config),
        Involvement::decl(&config),
        LanguageLevel::decl(&config),
        BiographyMode::decl(&config),
        SpokenLanguage::decl(&config),
        Interest::decl(&config),
        Biography::decl(&config),
        PersonIdentity::decl(&config),
        PersonalityDescriptor::decl(&config),
        PersonalityDescription::decl(&config),
        ExceptionalTrait::decl(&config),
        TraitStatus::decl(&config),
        TraitState::decl(&config),
        TraitTransition::decl(&config),
        TraitOverview::decl(&config),
        MovementAttributes::decl(&config),
        PhysicalityAttributes::decl(&config),
        RingcraftAttributes::decl(&config),
        PsychologyAttributes::decl(&config),
        FundamentalsAttributes::decl(&config),
        EntertainmentAttributes::decl(&config),
        WrestlerAttributes::decl(&config),
        Discipline::decl(&config),
        Tempo::decl(&config),
        Structure::decl(&config),
        Presentation::decl(&config),
        Contact::decl(&config),
        RiskApproach::decl(&config),
        Specialisation::decl(&config),
        WrestlingApproach::decl(&config),
        DisciplineEvidence::decl(&config),
        WrestlingStyleProfile::decl(&config),
        GroupScores::decl(&config),
        DisciplineFit::decl(&config),
        WrestlingSummary::decl(&config),
        RelationshipMemoryKind::decl(&config),
        InteractionKind::decl(&config),
        InteractionTone::decl(&config),
        RelationshipSignals::decl(&config),
        RelationshipMemoryView::decl(&config),
        PersonalRelationshipView::decl(&config),
        ManagementRelationshipView::decl(&config),
        RelationshipDelta::decl(&config),
        InteractionTarget::decl(&config),
        InteractionTargetPage::decl(&config),
        InteractionOption::decl(&config),
        InteractionOutcome::decl(&config),
        RelationshipProfile::decl(&config),
        InteractionRequest::decl(&config),
        Condition::decl(&config),
        Move::decl(&config),
        Worker::decl(&config),
        RosterRow::decl(&config),
        RosterPage::decl(&config),
        RoadAgent::decl(&config),
        BeatKind::decl(&config),
        PlannedBeat::decl(&config),
        Finish::decl(&config),
        MatchPlan::decl(&config),
        ParticipantSlot::decl(&config),
        MatchSide::decl(&config),
        ParticipationRule::decl(&config),
        VictoryRule::decl(&config),
        MatchRules::decl(&config),
        DecisionMethod::decl(&config),
        BookedResult::decl(&config),
        MatchDefinition::decl(&config),
        AnglePlan::decl(&config),
        SegmentPlan::decl(&config),
        Segment::decl(&config),
        ShowCard::decl(&config),
        SaveSegmentRequest::decl(&config),
        Cohort::decl(&config),
        CrowdState::decl(&config),
        Performance::decl(&config),
        SimEvent::decl(&config),
        WorkerChange::decl(&config),
        SegmentReport::decl(&config),
        MediaPost::decl(&config),
        ShowReport::decl(&config),
        MatchView::decl(&config),
        LiveView::decl(&config),
        InstructionKind::decl(&config),
        LiveInstruction::decl(&config),
        AdvanceRequest::decl(&config),
        AgentAdvice::decl(&config),
        CareerOffice::decl(&config),
        ProfileCompany::decl(&config),
        ProfileParticipant::decl(&config),
        ProfileAppearanceKind::decl(&config),
        ProfileAppearance::decl(&config),
        ProfileBooking::decl(&config),
        WorkerProfile::decl(&config),
        NewsItem::decl(&config),
        NewsPage::decl(&config),
    ]
    .map(|declaration| format!("export {declaration}"));

    format!(
        "// Generated by wm-domain. Run `cargo run -p wm-domain --bin export-contracts`.\n\
         // Do not edit by hand.\n\n{}\n",
        declarations.join("\n\n")
    )
}

fn output_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("packages/contracts/src/generated.ts")
}

fn main() -> ExitCode {
    let check = env::args().skip(1).any(|argument| argument == "--check");
    let expected = generated_contracts();
    let output = output_path();

    if check {
        return match fs::read_to_string(&output) {
            Ok(actual) if actual == expected => ExitCode::SUCCESS,
            Ok(_) => {
                eprintln!("generated TypeScript contracts are out of date");
                ExitCode::FAILURE
            }
            Err(error) => {
                eprintln!("could not read generated TypeScript contracts: {error}");
                ExitCode::FAILURE
            }
        };
    }

    if let Some(parent) = output.parent()
        && let Err(error) = fs::create_dir_all(parent)
    {
        eprintln!("could not create contract output directory: {error}");
        return ExitCode::FAILURE;
    }

    match fs::write(&output, expected) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("could not write TypeScript contracts: {error}");
            ExitCode::FAILURE
        }
    }
}
