use tauri::{command, AppHandle, Wry};

use crate::{
    db::path_talents::{self, FullPathTalent},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_full_talent(app: AppHandle<Wry>, id: i64) -> WWResult<FullPathTalent> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(path_talents::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_ancestry_talents(
    app: AppHandle<Wry>,
    ancestry_id: i64,
) -> WWResult<Vec<FullPathTalent>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(path_talents::get_for_ancestry(&db_state.pool, ancestry_id).await?)
}
