use tauri::{command, AppHandle, Wry};

use crate::{
    db::languages::{self, Language},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_non_secret_languages(app: AppHandle<Wry>) -> WWResult<Vec<Language>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(languages::get_all_non_secret(&db_state.pool).await?)
}
