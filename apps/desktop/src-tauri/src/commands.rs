use tauri::State;
use wm_domain::{CreateGameRequest, IpcError, PromotionOverview, SaveSummary};
use wm_persistence::{PersistenceError, SaveRepository};

pub(crate) async fn run_storage<T, F>(
    repository: SaveRepository,
    operation: F,
) -> Result<T, IpcError>
where
    T: Send + 'static,
    F: FnOnce(SaveRepository) -> Result<T, PersistenceError> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(move || {
        operation(repository).map_err(|error| {
            tracing::warn!(code = error.code(), error = ?error, "save_operation_failed");
            error.to_ipc_error()
        })
    })
    .await
    .map_err(|error| {
        tracing::error!(error = ?error, "storage_worker_failed");
        IpcError {
            code: "worker_failed".into(),
            message: "The save operation was interrupted. Please try again.".into(),
        }
    })?
}

#[tauri::command]
pub async fn create_game(
    repository: State<'_, SaveRepository>,
    request: CreateGameRequest,
) -> Result<PromotionOverview, IpcError> {
    run_storage(repository.inner().clone(), |store| {
        store.create_game(request)
    })
    .await
}

#[tauri::command]
pub async fn load_game(
    repository: State<'_, SaveRepository>,
    save_id: String,
) -> Result<PromotionOverview, IpcError> {
    run_storage(repository.inner().clone(), move |store| {
        store.load_game(&save_id)
    })
    .await
}

#[tauri::command]
pub async fn get_promotion_overview(
    repository: State<'_, SaveRepository>,
    save_id: String,
) -> Result<PromotionOverview, IpcError> {
    run_storage(repository.inner().clone(), move |store| {
        store.get_promotion_overview(&save_id)
    })
    .await
}

#[tauri::command]
pub async fn list_saves(
    repository: State<'_, SaveRepository>,
) -> Result<Vec<SaveSummary>, IpcError> {
    run_storage(repository.inner().clone(), |store| store.list_saves()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocking_adapter_returns_real_data_and_serialisable_errors() {
        let directory = tempfile::tempdir().unwrap();
        let repository = SaveRepository::new(directory.path());
        let request = CreateGameRequest {
            save_id: "adapter-test".into(),
            seed: "42".into(),
        };
        let created = tauri::async_runtime::block_on(run_storage(repository.clone(), |store| {
            store.create_game(request)
        }))
        .unwrap();
        assert_eq!(created.name, "Ultimate Wrestling Federation");
        let missing = tauri::async_runtime::block_on(run_storage(repository, |store| {
            store.load_game("missing")
        }))
        .unwrap_err();
        assert_eq!(missing.code, "save_not_found");
    }
}
