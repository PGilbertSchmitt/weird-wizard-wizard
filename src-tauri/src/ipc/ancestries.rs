use tauri::{command, AppHandle, Wry};

use crate::{
    db::ancestries::{self, FullAncestry},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_full_ancestry(app: AppHandle<Wry>, id: i64) -> WWResult<FullAncestry> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(ancestries::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_all_ancestries(app: AppHandle<Wry>) -> WWResult<Vec<FullAncestry>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(ancestries::get_all(&db_state.pool).await?)
}
