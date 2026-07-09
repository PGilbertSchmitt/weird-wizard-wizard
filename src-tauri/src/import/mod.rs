use csv::Reader;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use ts_rs::TS;

use crate::{WWError, WWResult};

mod init_seed;
mod run_seed;

pub use init_seed::initialize_seed_import;
pub use run_seed::run_seed_import;

pub(super) fn extract_rows<T>(filepath: PathBuf) -> WWResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let mut reader = Reader::from_path(&filepath).map_err(|_| {
        WWError::Generic(format!(
            "Could not find a csv in the zip file with name {:?}",
            filepath.as_path().file_name().unwrap()
        ))
    })?;
    let mut output = Vec::new();
    for result in reader.deserialize() {
        let record: T = result?;
        output.push(record);
    }
    Ok(output)
}

pub fn pipe_separate(column: &Option<String>) -> Vec<String> {
    match column {
        Some(s) => s.split("|").map(|part| part.trim().to_owned()).collect(),
        None => Vec::new(),
    }
}

pub type NameToId = HashMap<String, i64>;

const AFFIRMATIVE_STRINGS: [&'static str; 3] = ["Y", "YES", "TRUE"];

pub fn is_affirmative(value: Option<&str>) -> bool {
    value.map_or(false, |s| {
        let s = s.to_ascii_uppercase();
        AFFIRMATIVE_STRINGS.iter().any(|v| v == &s)
    })
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ImportEvent {
    Ready(ImportSummary),
    Progress(ProgressPayload),
    Done,
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "import.ts")]
pub struct ProgressPayload(usize, usize);

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

impl ImportSummary {
    pub fn total_records(&self) -> usize {
        self.ancestries
            + self.languages
            + self.speed_traits
            + self.senses
            + self.profession_categories
            + self.professions
            + self.traditions
            + self.magic_talents
            + self.magic_spells
            + self.novice_paths
            + self.novice_levels
            + self.expert_paths
            + self.expert_levels
            + self.master_paths
            + self.master_levels
            + self.path_talents
            + self.options
            + self.tables
    }
}

/* CSV Records */

#[derive(Deserialize, Debug)]
pub(super) struct AncestryRow {
    pub ancestry: String,
    pub base_size: String,
    pub base_speed: u32,
    pub add_health: Option<u32>,
    pub add_nat_def: Option<u32>,
    pub languages: Option<String>,
    pub speed_traits: Option<String>,
    pub senses: Option<String>,
    pub immunities: Option<String>,
    pub descriptor: Option<String>,
    pub traits: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct LanguageRow {
    pub languages: String,
    pub description: String,
    pub secret: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct SpeedTraitRow {
    pub name: String,
    pub unit: Option<String>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct SenseRow {
    pub name: String,
    pub unit: Option<String>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct ProfessionCategoryRow {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct ProfessionRow {
    pub name: String,
    pub category: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct TraditionRow {
    pub name: String,
    pub table: Option<String>,
    pub blurb: String,
    pub special_info: Option<String>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct MagicTalentRow {
    pub tradition: String,
    pub talent_name: String,
    pub charges: Option<String>,
    pub restore: String,
    pub activate: String,
    pub table: Option<String>,
    pub options: Option<String>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct MagicSpellRow {
    pub name: String,
    pub tradition: String,
    pub path_type: String,
    pub castings: u32,
    pub duration: String,
    pub target: String,
    pub condition: Option<String>,
    pub ritual: String,
    pub table: Option<String>,
    pub options: Option<String>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct NovicePathRow {
    pub name: String,
    pub description: String,
    pub init_scores_lvl_1: String,
    pub origin_locked: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct NoviceLevelRow {
    pub path: String,
    pub level: u32,
    pub health: u32,
    pub nat_def: Option<u32>,
    pub armed_def: Option<u32>,
    pub speed: Option<u32>,
    pub bonus_dmg: Option<u32>,
    pub trad_choices: Option<u32>,
    pub novice_spells: Option<u32>,
    pub expert_spells: Option<u32>,
    pub master_spells: Option<u32>,
    pub lang_choices: Option<u32>,
    pub languages: Option<String>,
    pub speed_traits: Option<String>,
    pub size: Option<String>,
    pub talents: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct ExpertPathRow {
    pub name: String,
    pub sub_path: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct ExpertLevelRow {
    pub path: String,
    pub level: u32,
    pub health: u32,
    pub nat_def: Option<u32>,
    pub armed_def: Option<u32>,
    pub speed: Option<u32>,
    pub bonus_dmg: Option<u32>,
    pub trad_choices: Option<u32>,
    pub novice_spells: Option<u32>,
    pub expert_spells: Option<u32>,
    pub master_spells: Option<u32>,
    pub lang_choices: Option<u32>,
    pub languages: Option<String>,
    pub speed_traits: Option<String>,
    pub size: Option<String>,
    pub talents: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct MasterPathRow {
    pub name: String,
    pub sub_path: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct MasterLevelRow {
    pub path: String,
    pub level: u32,
    pub health: u32,
    pub nat_def: Option<u32>,
    pub armed_def: Option<u32>,
    pub speed: Option<u32>,
    pub bonus_dmg: Option<u32>,
    pub trad_choices: Option<u32>,
    pub traditions: Option<String>,
    pub novice_spells: Option<u32>,
    pub expert_spells: Option<u32>,
    pub master_spells: Option<u32>,
    pub lang_choices: Option<u32>,
    pub languages: Option<String>,
    pub speed_traits: Option<String>,
    pub size: Option<String>,
    pub talents: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct PathTalentRow {
    pub name: String,
    pub source: String,
    pub magical: Option<String>,
    pub charges: Option<String>,
    pub restore: String,
    pub activate: String,
    pub table: Option<String>,
    pub options: Option<String>,
    // Should not be optional, but my current file is spotty
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct OptionRow {
    pub options_id: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct TableRow {
    pub table_id: String,
    pub key: String,
    pub value: String,
    pub table_type: Option<String>,
}
