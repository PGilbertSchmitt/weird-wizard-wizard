use core::time;
use std::{fs::File, path::PathBuf, sync::Mutex, thread::sleep};

use crate::{
    WWError, WWResult, import::{self, AncestryRow, LanguageRow, ProfessionCategoryRow, ProfessionRow, SenseRow, SpeedTraitRow}, ipc::{EmitChannel, emit}, store::WWAppData,
};

use serde::Serialize;
use tauri::{command, AppHandle, Manager, Wry};
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
    sleep(time::Duration::from_millis(500));

    let import_data = ImportData {
        ancestries: import::extract_rows::<AncestryRow>(join(&tmp_dir_path, "ancestries.csv"))?,
        languages: import::extract_rows::<LanguageRow>(join(&tmp_dir_path, "languages.csv"))?,
        speed_traits: import::extract_rows::<SpeedTraitRow>(join(&tmp_dir_path, "speed_traits.csv"))?,
        senses: import::extract_rows::<SenseRow>(join(&tmp_dir_path, "senses.csv"))?,
        profession_categories: import::extract_rows::<ProfessionCategoryRow>(join(&tmp_dir_path, "profession_categories.csv"))?,
        professions: import::extract_rows::<ProfessionRow>(join(&tmp_dir_path, "professions.csv"))?,
    };
    let import_summary = import_data.summary();
    state.import_data = Some(import_data);

    emit(
        &app,
        EmitChannel::IMPORT,
        &Ok(ImportEvent::Ready(import_summary))
        .into(),
    )?;

    // emit(
    //     &app,
    //     EmitChannel::IMPORT,
    //     &Ok(ImportEvent::Progress(ProgressPayload(19, 32))).into(),
    // )?;

    // emit(&app, EmitChannel::IMPORT, &Ok(ImportEvent::Done).into())?;

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
    Ready(ImportSummary),
    Progress(ProgressPayload),
    Done, // An empty payload makes the TS type easier to work with
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts")]
struct ProgressPayload(u32, u32);

pub struct ImportData {
    ancestries: Vec<AncestryRow>,
    languages: Vec<LanguageRow>,
    speed_traits: Vec<SpeedTraitRow>,
    senses: Vec<SenseRow>,
    profession_categories: Vec<ProfessionCategoryRow>,
    professions: Vec<ProfessionRow>,
}

impl ImportData {
    pub fn summary(&self) -> ImportSummary {
        ImportSummary {
            ancestries: self.ancestries.len(),
            languages: self.languages.len(),
            speed_traits: self.speed_traits.len(),
            senses: self.senses.len(),
            profession_categories: self.profession_categories.len(),
            professions: self.professions.len(),
        }
    }
}

#[derive(TS, Serialize)]
#[ts(export, export_to="import.ts")]
pub struct ImportSummary {
    ancestries: usize,
    languages: usize,
    speed_traits: usize,
    senses: usize,
    profession_categories: usize,
    professions: usize,
}
