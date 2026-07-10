use tauri::{command, AppHandle, Wry};

use crate::{
    db::{info_tables::FullInfoTable, option_blocks::FullOptionBlock},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_table(app: AppHandle<Wry>, id: i64) -> WWResult<FullInfoTable> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(FullInfoTable::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_option_block(app: AppHandle<Wry>, id: i64) -> WWResult<FullOptionBlock> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(FullOptionBlock::get(&db_state.pool, id).await?)
}
