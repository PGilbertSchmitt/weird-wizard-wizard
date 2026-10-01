use tauri::{command, AppHandle, Wry};

use crate::{
    db::{
        character_choices,
        choice_selections::{self, ChoiceTable},
    },
    modifiers::FullModifier,
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_choice_table(app: AppHandle<Wry>, name: String) -> WWResult<ChoiceTable> {
    let db_state = get_database(&app)?;
    choice_selections::get_choice_table(&db_state.pool, name).await
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
