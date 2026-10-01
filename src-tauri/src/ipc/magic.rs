use tauri::{command, AppHandle, Wry};

use crate::{
    db::{
        magic_talents::{self, FullMagicTalent},
        spells::{self, FullSpell},
        traditions::{self, FullTradition, TraditionIndexItem},
    },
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_tradition(app: AppHandle<Wry>, id: i64) -> WWResult<FullTradition> {
    let db_state = get_database(&app)?;
    Ok(traditions::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_tradition_index(app: AppHandle<Wry>) -> WWResult<Vec<TraditionIndexItem>> {
    let db_state = get_database(&app)?;
    Ok(traditions::get_index(&db_state.pool).await?)
}

#[command]
pub async fn get_spell(app: AppHandle<Wry>, id: i64) -> WWResult<FullSpell> {
    let db_state = get_database(&app)?;
    Ok(spells::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_spells_for_tradition(
    app: AppHandle<Wry>,
    tradition_id: i64,
) -> WWResult<Vec<FullSpell>> {
    let db_state = get_database(&app)?;
    Ok(spells::get_for_tradition(&db_state.pool, tradition_id).await?)
}

#[command]
pub async fn get_magic_talent(app: AppHandle<Wry>, id: i64) -> WWResult<FullMagicTalent> {
    let db_state = get_database(&app)?;
    Ok(magic_talents::get(&db_state.pool, id).await?)
}

#[command]
pub async fn get_magic_talents_for_tradition(
    app: AppHandle<Wry>,
    tradition_id: i64,
) -> WWResult<Vec<FullMagicTalent>> {
    let db_state = get_database(&app)?;
    Ok(magic_talents::get_for_tradition(&db_state.pool, tradition_id).await?)
}
