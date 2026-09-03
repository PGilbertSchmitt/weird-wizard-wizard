use tauri::{command, AppHandle, Wry};

use crate::{
    db::{
        magic_talents::{self, FullMagicTalent},
        spells::{self, FullSpell},
        traditions::{self, FullTradition},
    },
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_tradition(app: AppHandle<Wry>, id: i64) -> WWResult<FullTradition> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(traditions::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_spell(app: AppHandle<Wry>, id: i64) -> WWResult<FullSpell> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(spells::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_magic_talent(app: AppHandle<Wry>, id: i64) -> WWResult<FullMagicTalent> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(magic_talents::get(&db_state.pool, id).await?)
}
