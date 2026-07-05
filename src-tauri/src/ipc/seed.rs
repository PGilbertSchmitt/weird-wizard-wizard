use core::time;
use std::{fs::File, path::PathBuf, sync::Mutex, thread::sleep};

use crate::{
    import,
    ipc::{emit, EmitChannel},
    store::{ImportData, WWAppData},
    WWError, WWResult,
};

use serde::Serialize;
use tauri::{
    command,
    webview::cookie::time::format_description::well_known::iso8601::FormattedComponents::Time,
    AppHandle, Manager, Wry,
};
use ts_rs::TS;

// This is potentially long running, so it communicates using events
#[command]
pub async fn init_seed(app: AppHandle<Wry>, filepath: String) {
    if let WWResult::Err(e) = initialize_import_seed(&app, filepath).await {
        emit(&app, EmitChannel::IMPORT, &e.into()).unwrap()
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

    let ancestries = import::extract_ancestries(join(&tmp_dir_path, "ancestries.csv"))?;
    let ancestry_count = ancestries.len();

    let import_data = ImportData { ancestries };
    state.import_data = Some(import_data);

    emit(
        &app,
        EmitChannel::IMPORT,
        &Ok(ImportEvent::Ready(ReadyPayload {
            ancestries: ancestry_count,
        }))
        .into(),
    )?;

    emit(
        &app,
        EmitChannel::IMPORT,
        &Ok(ImportEvent::Progress(ProgressPayload(
            19,
            32,
        )))
        .into(),
    )?;

    emit(&app, EmitChannel::IMPORT, &Ok(ImportEvent::Done).into())?;

    Ok(())
}

fn join(path: &PathBuf, filename: &str) -> PathBuf {
    let mut next_path = path.clone();
    next_path.push(filename);
    next_path
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
enum ImportEvent {
    Ready(ReadyPayload),
    Progress(ProgressPayload),
    Done, // An empty payload makes the TS type easier to work with
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts")]
struct ReadyPayload {
    ancestries: usize,
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts")]
struct ProgressPayload(u32, u32);
