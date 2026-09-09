use tauri::{command, AppHandle, Wry};

use crate::{
    db::profession_categories::{self, FullProfessionCategory},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_all_professions(app: AppHandle<Wry>) -> WWResult<Vec<FullProfessionCategory>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(profession_categories::get_all(&db_state.pool).await?)
}
