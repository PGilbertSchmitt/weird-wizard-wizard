use std::{fs, path::PathBuf};

use csv::Reader;
use serde::Deserialize;

use crate::WWResult;

#[derive(Deserialize, Debug)]
pub struct AncestryRow {
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

pub fn extract_ancestries(filepath: PathBuf) -> WWResult<Vec<AncestryRow>> {
    let mut reader = Reader::from_path(filepath)?;

    let mut output = Vec::new();
    for result in reader.deserialize() {
        let record: AncestryRow = result?;
        output.push(record);
    }

    Ok(output)
}
