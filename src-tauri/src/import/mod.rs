use serde::{Deserialize, de::DeserializeOwned};
use std::path::PathBuf;
use csv::Reader;

use crate::WWResult;

pub(super) fn extract_rows<T>(filepath: PathBuf) -> WWResult<Vec<T>>
where
    T: DeserializeOwned
{
    let mut reader = Reader::from_path(filepath)?;
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
        None => Vec::new()
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
    languages: String,
    description: String,
    secret: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct SpeedTraitRow {
    name: String,
    units: Option<String>,
    description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct SenseRow {
    name: String,
    description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct ProfessionCategoryRow {
    name: String,
    description: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct ProfessionRow {
    name: String,
    category: String,
    description: String,
}
