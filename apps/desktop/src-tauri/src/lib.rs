mod commands;
mod game_commands;

use tauri::Manager;
use wm_persistence::SaveRepository;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wm_desktop=info".into()),
        )
        .with_target(true)
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let saves = app.path().app_data_dir()?.join("saves");
            // Tests isolate their files; release builds always use application data.
            #[cfg(debug_assertions)]
            let saves = std::env::var_os("WM_SAVE_DIR")
                .map(std::path::PathBuf::from)
                .unwrap_or(saves);
            app.manage(SaveRepository::new(saves));
            tracing::info!(engine = wm_domain::CURRENT_ENGINE_VERSION, "desktop_ready");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_game,
            commands::load_game,
            commands::get_promotion_overview,
            commands::list_saves,
            game_commands::career_office,
            game_commands::roster_page,
            game_commands::worker_search,
            game_commands::worker_filter_options,
            game_commands::worker_discovery_lists,
            game_commands::save_worker_view,
            game_commands::delete_worker_view,
            game_commands::create_worker_shortlist,
            game_commands::delete_worker_shortlist,
            game_commands::set_worker_shortlist_member,
            game_commands::set_worker_blacklisted,
            game_commands::compare_workers,
            game_commands::worker_profile,
            game_commands::relationship_targets,
            game_commands::interact_with_worker,
            game_commands::show_card,
            game_commands::agent_advice,
            game_commands::save_segment,
            game_commands::rearrange_card,
            game_commands::delete_segment,
            game_commands::start_show,
            game_commands::live_show,
            game_commands::advance_show,
            game_commands::live_instruction,
            game_commands::show_report,
            game_commands::continue_day,
            game_commands::toggle_fullscreen,
            game_commands::exit_game,
            game_commands::news_page,
            game_commands::set_news_read,
        ])
        .run(tauri::generate_context!())
        .expect("Wrestling Manager could not start");
}
