use tauri::{command, AppHandle, Wry};

use crate::{
    WWResult, db::{character_choices, characters::{self, CharacterIndexItem, CreateCharacter, FullCharacter}}, modifiers::FullModifier, store::get_database,
};

#[command]
pub async fn get_character_index(app: AppHandle<Wry>) -> WWResult<Vec<CharacterIndexItem>> {
    let db_state = get_database(&app)?;
    characters::get_index(&db_state.pool).await
}

#[command]
pub async fn create_character(
    app: AppHandle<Wry>,
    character_info: CreateCharacter,
) -> WWResult<i64> {
    let db_state = get_database(&app)?;
    characters::create_character(&db_state.pool, character_info).await
}

#[command]
pub async fn get_full_character(app: AppHandle<Wry>, id: i64) -> WWResult<FullCharacter> {
    let db_state = get_database(&app)?;
    characters::get(&db_state.pool, id).await
}

#[command]
pub async fn update_character_health(
    app: AppHandle<Wry>,
    id: i64,
    health: i64,
    damage: i64,
) -> WWResult<()> {
    let db_state = get_database(&app)?;
    characters::update_health(&db_state.pool, id, health, damage).await
}

#[command]
pub async fn update_character_level(app: AppHandle<Wry>, id: i64, level: i64) -> WWResult<()> {
    let db_state = get_database(&app)?;
    characters::update_level(&db_state.pool, id, level).await
}

#[command]
pub async fn update_character_path(app: AppHandle<Wry>, id: i64, path_id: i64) -> WWResult<()> {
    let db_state = get_database(&app)?;
    characters::set_path(&db_state.pool, id, path_id).await
}

#[command]
pub async fn save_choice(
    app: AppHandle<Wry>,
    character_id: i64,
    modifier: FullModifier,
    values: Vec<String>,
) -> WWResult<()> {
    let db_state = get_database(&app)?;
    character_choices::save_choices(&db_state.pool, character_id, modifier, values).await
}

#[command]
pub async fn delete_choice(
    app: AppHandle<Wry>,
    character_id: i64,
    modifier: FullModifier,
) -> WWResult<()> {
    let db_state = get_database(&app)?;
    character_choices::delete_choice(&db_state.pool, character_id, modifier).await
}
