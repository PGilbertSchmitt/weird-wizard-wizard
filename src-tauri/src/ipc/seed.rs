use core::time;
use std::{fs::File, sync::Mutex, thread::sleep};

use crate::{WWError, WWResult, ipc::{EmitChannel, emit}, store::WWAppData};

use tauri::{command, AppHandle, Manager, Wry};

// This is potentially long running, so it communicates using events
#[command]
pub async fn init_seed(app: AppHandle<Wry>, filepath: String) {
    if let WWResult::Err(e) = initialize_import_seed(&app, filepath).await {
        emit(&app, EmitChannel::IMPORT, e.into()).unwrap()
    }
}

async fn initialize_import_seed(app: &AppHandle<Wry>, filepath: String) -> WWResult<()> {
    let file = File::open(filepath)?;
    let state: tauri::State<'_, Mutex<WWAppData>> = app.state();
    let mut state = state
        .lock()
        .map_err(|_| WWError::Generic("Poisoned Mutex error".to_owned()))?;
    let tmp_dir_path = state.add_import_dir()?;
    let mut zipfile = zip::ZipArchive::new(file)?;
    zipfile.extract(&tmp_dir_path).unwrap();
    println!("Extracted contents to {:?}", tmp_dir_path);

    // TODO: process zipfile contents to send resource summary with READY signal
    sleep(time::Duration::from_millis(1000));

    emit(&app, EmitChannel::IMPORT, Ok("READY").into())?;

    Ok(())
}
