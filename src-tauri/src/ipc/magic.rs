use tauri::{command, AppHandle, Wry};

use crate::{
    db::{magic_talents::FullMagicTalent, spells::FullSpell, traditions::FullTradition},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_tradition(app: AppHandle<Wry>, id: i64) -> WWResult<FullTradition> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(FullTradition::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_spell(app: AppHandle<Wry>, id: i64) -> WWResult<FullSpell> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(FullSpell::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_magic_talent(app: AppHandle<Wry>, id: i64) -> WWResult<FullMagicTalent> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(FullMagicTalent::get(&db_state.pool, id).await?)
}
