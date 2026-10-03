use tauri::{command, AppHandle, Wry};

use crate::{
    db::choice_selections::{self, ChoiceTable, FullChoice},
    store::get_database,
    WWResult,
};

#[command]
pub async fn get_choice_table(app: AppHandle<Wry>, name: String) -> WWResult<ChoiceTable> {
    let db_state = get_database(&app)?;
    choice_selections::get_choice_table(&db_state.pool, name).await
}

#[command]
pub async fn get_choice_selection(app: AppHandle<Wry>, id: i64) -> WWResult<FullChoice> {
    let db_state = get_database(&app)?;
    choice_selections::get_full_choice_selection(&db_state.pool, id).await
}
