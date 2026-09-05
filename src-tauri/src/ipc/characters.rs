use tauri::{command, AppHandle, Wry};

use crate::{
    db::characters::{self, CharacterIndexItem},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_character_index(app: AppHandle<Wry>) -> WWResult<Vec<CharacterIndexItem>> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    Ok(characters::get_index(&db_state.pool).await?)
}

