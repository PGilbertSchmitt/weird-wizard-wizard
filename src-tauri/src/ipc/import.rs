use tauri::{command, AppHandle, Wry};

use crate::{
    WWResult, db::seed_data, import::{initialize_seed_import, run_seed_import}, ipc::{EmitChannel, emit}, store::get_database,
};

// This is potentially long running, so it communicates using events
#[command]
pub async fn init_seed(app: AppHandle<Wry>, filepath: String) {
    if let WWResult::Err(e) = initialize_seed_import(&app, filepath).await {
        emit(&app, EmitChannel::IMPORT, &e.into()).unwrap()
    }
}

#[command]
pub async fn run_seed(app: AppHandle<Wry>) {
    if let WWResult::Err(e) = run_seed_import(&app).await {
        emit(&app, EmitChannel::IMPORT, &e.into()).unwrap()
    }
}

#[command]
pub async fn check_seed(app: AppHandle<Wry>) -> WWResult<bool> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(seed_data::is_seeded(&db_state.pool).await?)
}
