use tauri::{command, AppHandle, Wry};

use crate::{
    WWResult, db::ancestries::{self, FullAncestry}, store::get_database,
};

#[command]
pub async fn get_full_ancestry(app: AppHandle<Wry>, id: i64) -> WWResult<FullAncestry> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(ancestries::get_full_ancestry(&db_state.pool, id).await?)
}
