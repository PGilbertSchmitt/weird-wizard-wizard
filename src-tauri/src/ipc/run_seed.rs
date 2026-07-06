use tauri::{AppHandle, Wry, command};

use crate::{WWError, WWResult, ipc::{EmitChannel, ImportEvent, emit}, store::{get_app_data_state, get_database}};

#[command]
pub async fn run_seed(app: AppHandle<Wry>) {
    if let WWResult::Err(e) = run_seed_import(&app).await {
        emit(&app, super::EmitChannel::IMPORT, &e.into()).unwrap()
    }
}

async fn run_seed_import(app: &AppHandle<Wry>) -> WWResult<()> {
    let import_state = get_app_data_state(&app)?;
    let import_state = import_state
        .lock()
        .await;

    let db_state = get_database(&app)?;
    let db_state = db_state
        .lock()
        .await;
    
    if let Some(import_data) = &import_state.import_data {

        let db = &db_state.pool;
        for lang in &import_data.languages {
            let secret = lang.secret.as_deref() == Some("TRUE");
            let _ = sqlx::query!(
                r#"
                INSERT INTO languages ( name, description, secret ) VALUES (
                    ?, ?, ?
                )
                "#,
                lang.languages,
                lang.description,
                secret,
            ).execute(db).await?;
        }

        emit(&app, EmitChannel::IMPORT, &Ok(ImportEvent::Done).into())?;
        
        Ok(())
    } else {
        Err(WWError::Generic("No".to_owned()))
    }
}


