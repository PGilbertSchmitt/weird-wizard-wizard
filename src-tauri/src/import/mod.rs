use csv::Reader;
use serde::{de::DeserializeOwned, Deserialize};
use std::path::PathBuf;

use crate::{WWError, WWResult};

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

pub fn pipe_separate(column: Option<String>) -> Vec<String> {
    match column {
        Some(s) => s.split("|").map(|part| part.trim().to_owned()).collect(),
        None => Vec::new(),
    }
}

/* CSV Records */

#[derive(Deserialize, Debug)]
pub(super) struct AncestryRow {
    ancestry: String,
    base_size: String,
    base_speed: u32,
    base_health: Option<u32>,
    base_nat_def: Option<u32>,
    languages: Option<String>,
    speed_traits: Option<String>,
    senses: Option<String>,
    immunities: Option<String>,
    descriptor: Option<String>,
    traits: Option<String>,
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
    pub units: Option<String>,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct SenseRow {
    pub name: String,
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
