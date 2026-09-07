use tauri::{command, AppHandle, Wry};

use crate::{
    db::paths::{self, FullPath, NovicePath, PathIndexItem},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_path_index(app: AppHandle<Wry>) -> WWResult<Vec<PathIndexItem>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(paths::get_path_index(&db_state.pool).await?)
}

#[command]
pub async fn get_paths_for_kind_and_category(
    app: AppHandle<Wry>,
    kind: String,
    category: String,
) -> WWResult<Vec<FullPath>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(paths::get_for_kind_and_category(&db_state.pool, kind, category).await?)
}

#[command]
pub async fn get_novice_path(app: AppHandle<Wry>, id: i64) -> WWResult<NovicePath> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(paths::get_novice_path(&db_state.pool, id).await?)
}

#[command]
pub async fn get_full_path(app: AppHandle<Wry>, id: i64) -> WWResult<FullPath> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(paths::get(&db_state.pool, id).await?)
}
