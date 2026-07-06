use std::{fs::File, path::PathBuf};

use crate::{
    WWResult, import::{
        self, AncestryRow, ExpertLevelRow, ExpertPathRow, LanguageRow, MagicSpellRow, MagicTalentRow, MasterLevelRow, MasterPathRow, NoviceLevelRow, NovicePathRow, OptionRow, PathTalentRow, ProfessionCategoryRow, ProfessionRow, SenseRow, SpeedTraitRow, TableRow, TraditionRow,
    }, ipc::{EmitChannel, emit}, store::get_app_data_state,
};

use serde::Serialize;
use tauri::{command, AppHandle, Wry};
use ts_rs::TS;

// This is potentially long running, so it communicates using events
#[command]
pub async fn init_seed(app: AppHandle<Wry>, filepath: String) {
    if let WWResult::Err(e) = initialize_seed_import(&app, filepath).await {
        emit(&app, EmitChannel::IMPORT, &e.into()).unwrap()
    }
}

async fn initialize_seed_import(app: &AppHandle<Wry>, filepath: String) -> WWResult<()> {
    let file = File::open(filepath)?;
    let state = get_app_data_state(&app)?;
    let mut state = state
        .lock()
        .await;
    let tmp_dir_path = state.add_import_dir()?;
    let mut zipfile = zip::ZipArchive::new(file)?;
    zipfile.extract(&tmp_dir_path).unwrap();
    println!("Extracted contents to {:?}", tmp_dir_path);

    let import_data = ImportData {
        ancestries: import::extract_rows::<AncestryRow>(join(&tmp_dir_path, "ancestries.csv"))?,
        languages: import::extract_rows::<LanguageRow>(join(&tmp_dir_path, "languages.csv"))?,
        speed_traits: import::extract_rows::<SpeedTraitRow>(join(
            &tmp_dir_path,
            "speed_traits.csv",
        ))?,
        senses: import::extract_rows::<SenseRow>(join(&tmp_dir_path, "senses.csv"))?,
        profession_categories: import::extract_rows::<ProfessionCategoryRow>(join(
            &tmp_dir_path,
            "profession_categories.csv",
        ))?,
        professions: import::extract_rows::<ProfessionRow>(join(&tmp_dir_path, "professions.csv"))?,
        traditions: import::extract_rows::<TraditionRow>(join(
            &tmp_dir_path,
            "magic_traditions.csv",
        ))?,
        magic_talents: import::extract_rows::<MagicTalentRow>(join(
            &tmp_dir_path,
            "magic_talents.csv",
        ))?,
        magic_spells: import::extract_rows::<MagicSpellRow>(join(
            &tmp_dir_path,
            "magic_spells.csv",
        ))?,
        novice_paths: import::extract_rows::<NovicePathRow>(join(
            &tmp_dir_path,
            "novice_paths.csv",
        ))?,
        novice_levels: import::extract_rows::<NoviceLevelRow>(join(
            &tmp_dir_path,
            "novice_levels.csv",
        ))?,
        expert_paths: import::extract_rows::<ExpertPathRow>(join(
            &tmp_dir_path,
            "expert_paths.csv",
        ))?,
        expert_levels: import::extract_rows::<ExpertLevelRow>(join(
            &tmp_dir_path,
            "expert_levels.csv",
        ))?,
        master_paths: import::extract_rows::<MasterPathRow>(join(
            &tmp_dir_path,
            "master_paths.csv",
        ))?,
        master_levels: import::extract_rows::<MasterLevelRow>(join(
            &tmp_dir_path,
            "master_levels.csv",
        ))?,
        path_talents: import::extract_rows::<PathTalentRow>(join(
            &tmp_dir_path,
            "path_talents.csv",
        ))?,
        options: import::extract_rows::<OptionRow>(join(
            &tmp_dir_path,
            "options.csv",
        ))?,
        tables: import::extract_rows::<TableRow>(join(
            &tmp_dir_path,
            "tables.csv",
        ))?,
    };
    let import_summary = import_data.summary();
    state.import_data = Some(import_data);

    emit(
        &app,
        EmitChannel::IMPORT,
        &Ok(ImportEvent::Ready(import_summary)).into(),
    )?;

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
pub enum ImportEvent {
    Ready(ImportSummary),
    Progress(ProgressPayload),
    Done, // An empty payload makes the TS type easier to work with
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts")]
struct ProgressPayload(u32, u32);

pub struct ImportData {
    pub ancestries: Vec<AncestryRow>,
    pub languages: Vec<LanguageRow>,
    pub speed_traits: Vec<SpeedTraitRow>,
    pub senses: Vec<SenseRow>,
    pub profession_categories: Vec<ProfessionCategoryRow>,
    pub professions: Vec<ProfessionRow>,
    pub traditions: Vec<TraditionRow>,
    pub magic_talents: Vec<MagicTalentRow>,
    pub magic_spells: Vec<MagicSpellRow>,
    pub novice_paths: Vec<NovicePathRow>,
    pub novice_levels: Vec<NoviceLevelRow>,
    pub expert_paths: Vec<ExpertPathRow>,
    pub expert_levels: Vec<ExpertLevelRow>,
    pub master_paths: Vec<MasterPathRow>,
    pub master_levels: Vec<MasterLevelRow>,
    pub path_talents: Vec<PathTalentRow>,
    pub options: Vec<OptionRow>,
    pub tables: Vec<TableRow>,
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
            traditions: self.traditions.len(),
            magic_talents: self.magic_talents.len(),
            magic_spells: self.magic_spells.len(),
            novice_paths: self.novice_paths.len(),
            novice_levels: self.novice_levels.len(),
            expert_paths: self.expert_paths.len(),
            expert_levels: self.expert_levels.len(),
            master_paths: self.master_paths.len(),
            master_levels: self.master_levels.len(),
            path_talents: self.path_talents.len(),
            options: self.options.len(),
            tables: self.tables.len(),
        }
    }
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts")]
pub struct ImportSummary {
    ancestries: usize,
    languages: usize,
    speed_traits: usize,
    senses: usize,
    profession_categories: usize,
    professions: usize,
    traditions: usize,
    magic_talents: usize,
    magic_spells: usize,
    novice_paths: usize,
    novice_levels: usize,
    expert_paths: usize,
    expert_levels: usize,
    master_paths: usize,
    master_levels: usize,
    path_talents: usize,
    options: usize,
    tables: usize,
}
