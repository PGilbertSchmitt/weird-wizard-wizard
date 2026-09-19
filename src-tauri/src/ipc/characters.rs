use tauri::{command, AppHandle, Wry};

use crate::{
    db::characters::{self, CharacterIndexItem, CreateCharacter, FullCharacter},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_character_index(app: AppHandle<Wry>) -> WWResult<Vec<CharacterIndexItem>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(characters::get_index(&db_state.pool).await?)
}

#[command]
pub async fn create_character(
    app: AppHandle<Wry>,
    character_info: CreateCharacter,
) -> WWResult<i64> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(characters::create_character(&db_state.pool, character_info).await?)
}

#[command]
pub async fn get_full_character(app: AppHandle<Wry>, id: i64) -> WWResult<FullCharacter> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(characters::get(&db_state.pool, id).await?)
}

#[command]
pub async fn update_character_health(
    app: AppHandle<Wry>,
    id: i64,
    health: i64,
    damage: i64,
) -> WWResult<()> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(characters::update_health(&db_state.pool, id, health, damage).await?)
}
