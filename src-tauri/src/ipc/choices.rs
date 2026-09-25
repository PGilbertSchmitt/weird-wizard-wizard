use tauri::{command, AppHandle, Wry};

use crate::{db::character_choices, modifiers::FullModifier, store::get_database, WWResult};

#[command]
pub async fn save_choice(
    app: AppHandle<Wry>,
    character_id: i64,
    modifier: FullModifier,
    values: Vec<String>,
) -> WWResult<()> {
    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;
    character_choices::save_choices(&db_state.pool, character_id, modifier, values).await
}
