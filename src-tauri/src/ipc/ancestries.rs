use tauri::{command, AppHandle, Wry};

use crate::{
    db::ancestries::{Ancestry, FullAncestry},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_ancestry(app: AppHandle<Wry>, id: i64) -> WWResult<Ancestry> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(Ancestry::get_ancestry(&db_state.pool, id).await?)
}

#[command]
pub async fn get_full_ancestry(app: AppHandle<Wry>, id: i64) -> WWResult<FullAncestry> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(Ancestry::get_full_ancestry(&db_state.pool, id).await?)
}
