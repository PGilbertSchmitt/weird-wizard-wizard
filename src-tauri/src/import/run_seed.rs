use std::collections::HashSet;

use tauri::{AppHandle, Wry};

use crate::{
    WWError, WWResult, db, import::pipe_separate, ipc::{EmitChannel, emit}, store::{get_app_data_state, get_database},
};

use super::ImportEvent;

pub async fn run_seed_import(app: &AppHandle<Wry>) -> WWResult<()> {
    let import_state = get_app_data_state(&app)?;
    let import_state = import_state.lock().await;

    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;

    if let Some(import_data) = &import_state.import_data {
        let pool = &db_state.pool;
        let mut tx = pool.begin().await?;
        let language_map = db::Language::insert_all(&mut tx, &import_data.languages).await?;
        let speed_trait_map = db::SpeedTrait::insert_all(&mut tx, &import_data.speed_traits).await?;
        let sense_map = db::Sense::insert_all(&mut tx, &import_data.senses).await?;
        let immunities: HashSet<String> = import_data
            .ancestries
            .iter()
            .flat_map(|anc| pipe_separate(&anc.immunities))
            .collect();
        let immunity_map = db::Immunity::insert_all(&mut tx, &immunities).await?;
        db::Ancestry::insert_all(&mut tx, &import_data.ancestries, language_map, speed_trait_map, sense_map, immunity_map).await?;
        tx.commit().await?;

        emit(&app, EmitChannel::IMPORT, &Ok(ImportEvent::Done).into())?;

        Ok(())
    } else {
        Err(WWError::Generic("No".to_owned()))
    }
}
