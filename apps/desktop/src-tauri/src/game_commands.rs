use crate::commands::run_storage;
use tauri::State;
use wm_domain::{
    IpcError,
    characters::{CharacterActionRequest, CharacterProfile, ProposeCharacterChangeRequest},
    discovery::{WorkerDiscoveryLists, WorkerFilterOptions, WorkerSearchPage, WorkerSearchRequest},
    game::*,
    relationships::{InteractionOutcome, InteractionRequest, InteractionTargetPage},
};
use wm_persistence::SaveRepository;

#[tauri::command]
pub async fn career_office(
    repository: State<'_, SaveRepository>,
    save_id: String,
) -> Result<CareerOffice, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.career_office(&save_id)
    })
    .await
}
#[tauri::command]
pub async fn roster_page(
    repository: State<'_, SaveRepository>,
    save_id: String,
    search: String,
    offset: u32,
    limit: u32,
) -> Result<RosterPage, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.roster_page(&save_id, &search, offset, limit)
    })
    .await
}
#[tauri::command]
pub async fn worker_search(
    repository: State<'_, SaveRepository>,
    save_id: String,
    request: WorkerSearchRequest,
) -> Result<WorkerSearchPage, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.worker_search(&save_id, request)
    })
    .await
}
#[tauri::command]
pub async fn worker_filter_options(
    repository: State<'_, SaveRepository>,
    save_id: String,
) -> Result<WorkerFilterOptions, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.worker_filter_options(&save_id)
    })
    .await
}
#[tauri::command]
pub async fn worker_discovery_lists(
    repository: State<'_, SaveRepository>,
    save_id: String,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.worker_discovery_lists(&save_id)
    })
    .await
}
#[tauri::command]
pub async fn save_worker_view(
    repository: State<'_, SaveRepository>,
    save_id: String,
    id: Option<i32>,
    name: String,
    request: WorkerSearchRequest,
    columns: Vec<String>,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.save_worker_view(&save_id, id, &name, request, columns)
    })
    .await
}
#[tauri::command]
pub async fn delete_worker_view(
    repository: State<'_, SaveRepository>,
    save_id: String,
    id: i32,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.delete_worker_view(&save_id, id)
    })
    .await
}
#[tauri::command]
pub async fn create_worker_shortlist(
    repository: State<'_, SaveRepository>,
    save_id: String,
    name: String,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.create_worker_shortlist(&save_id, &name)
    })
    .await
}
#[tauri::command]
pub async fn delete_worker_shortlist(
    repository: State<'_, SaveRepository>,
    save_id: String,
    id: i32,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.delete_worker_shortlist(&save_id, id)
    })
    .await
}
#[tauri::command]
pub async fn set_worker_shortlist_member(
    repository: State<'_, SaveRepository>,
    save_id: String,
    shortlist_id: i32,
    worker_id: String,
    included: bool,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.set_worker_shortlist_member(&save_id, shortlist_id, &worker_id, included)
    })
    .await
}
#[tauri::command]
pub async fn set_worker_blacklisted(
    repository: State<'_, SaveRepository>,
    save_id: String,
    worker_id: String,
    blacklisted: bool,
) -> Result<WorkerDiscoveryLists, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.set_worker_blacklisted(&save_id, &worker_id, blacklisted)
    })
    .await
}
#[tauri::command]
pub async fn compare_workers(
    repository: State<'_, SaveRepository>,
    save_id: String,
    ids: Vec<String>,
) -> Result<Vec<RosterRow>, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.compare_workers(&save_id, &ids)
    })
    .await
}
#[tauri::command]
pub async fn worker_profile(
    repository: State<'_, SaveRepository>,
    save_id: String,
    worker_id: String,
) -> Result<WorkerProfile, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.worker_profile(&save_id, &worker_id)
    })
    .await
}

#[tauri::command]
pub async fn propose_character_change(
    repository: State<'_, SaveRepository>,
    request: ProposeCharacterChangeRequest,
) -> Result<CharacterProfile, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.propose_character_change(request)
    })
    .await
}

#[tauri::command]
pub async fn launch_character_change(
    repository: State<'_, SaveRepository>,
    request: CharacterActionRequest,
) -> Result<CharacterProfile, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.launch_character_change(request)
    })
    .await
}

#[tauri::command]
pub async fn set_character_retired(
    repository: State<'_, SaveRepository>,
    request: CharacterActionRequest,
    retired: bool,
) -> Result<CharacterProfile, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.set_character_retired(request, retired)
    })
    .await
}
#[tauri::command]
pub async fn relationship_targets(
    repository: State<'_, SaveRepository>,
    save_id: String,
    worker_id: String,
    search: String,
    offset: u32,
    limit: u32,
) -> Result<InteractionTargetPage, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.interaction_targets(&save_id, &worker_id, &search, offset, limit)
    })
    .await
}
#[tauri::command]
pub async fn interact_with_worker(
    repository: State<'_, SaveRepository>,
    request: InteractionRequest,
) -> Result<InteractionOutcome, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.interact_with_worker(request)
    })
    .await
}
#[tauri::command]
pub async fn show_card(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
) -> Result<ShowCard, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.show_card(&save_id, show_id)
    })
    .await
}
#[tauri::command]
pub async fn agent_advice(
    repository: State<'_, SaveRepository>,
    save_id: String,
    plan: MatchPlan,
) -> Result<AgentAdvice, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.agent_advice(&save_id, plan)
    })
    .await
}
#[tauri::command]
pub async fn save_segment(
    repository: State<'_, SaveRepository>,
    request: SaveSegmentRequest,
) -> Result<ShowCard, IpcError> {
    run_storage(repository.inner().clone(), move |r| r.save_segment(request)).await
}
#[tauri::command]
pub async fn rearrange_card(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
    revision: i32,
    ids: Vec<i32>,
) -> Result<ShowCard, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.rearrange_card(&save_id, show_id, revision, ids)
    })
    .await
}
#[tauri::command]
pub async fn delete_segment(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
    revision: i32,
    segment_id: i32,
) -> Result<ShowCard, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.delete_segment(&save_id, show_id, revision, segment_id)
    })
    .await
}
#[tauri::command]
pub async fn start_show(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
) -> Result<LiveView, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.start_show(&save_id, show_id)
    })
    .await
}
#[tauri::command]
pub async fn live_show(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
) -> Result<LiveView, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.live_show(&save_id, show_id)
    })
    .await
}
#[tauri::command]
pub async fn advance_show(
    repository: State<'_, SaveRepository>,
    request: AdvanceRequest,
) -> Result<LiveView, IpcError> {
    run_storage(repository.inner().clone(), move |r| r.advance_show(request)).await
}
#[tauri::command]
pub async fn live_instruction(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
    instruction: LiveInstruction,
) -> Result<LiveView, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.live_instruction(&save_id, show_id, instruction)
    })
    .await
}
#[tauri::command]
pub async fn show_report(
    repository: State<'_, SaveRepository>,
    save_id: String,
    show_id: i32,
) -> Result<ShowReport, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.show_report(&save_id, show_id)
    })
    .await
}
#[tauri::command]
pub async fn continue_day(
    repository: State<'_, SaveRepository>,
    save_id: String,
) -> Result<CareerOffice, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.continue_day(&save_id)
    })
    .await
}
#[tauri::command]
pub fn toggle_fullscreen(window: tauri::WebviewWindow) -> Result<bool, IpcError> {
    let action = || -> tauri::Result<bool> {
        let next = !window.is_fullscreen()?;
        window.set_fullscreen(next)?;
        window.set_decorations(!next)?;
        Ok(next)
    };
    action().map_err(|_| IpcError {
        code: "window_error".into(),
        message: "Could not change display mode.".into(),
    })
}

#[tauri::command]
pub fn exit_game(window: tauri::WebviewWindow) -> Result<(), IpcError> {
    window.close().map_err(|_| IpcError {
        code: "window_error".into(),
        message: "Could not close Wrestling Manager.".into(),
    })
}

#[tauri::command]
pub async fn news_page(
    repository: State<'_, SaveRepository>,
    save_id: String,
    category: String,
    unread_only: bool,
    offset: u32,
    limit: u32,
) -> Result<NewsPage, IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.news_page(&save_id, &category, unread_only, offset, limit)
    })
    .await
}

#[tauri::command]
pub async fn set_news_read(
    repository: State<'_, SaveRepository>,
    save_id: String,
    id: i32,
    read: bool,
) -> Result<(), IpcError> {
    run_storage(repository.inner().clone(), move |r| {
        r.set_news_read(&save_id, id, read)
    })
    .await
}
