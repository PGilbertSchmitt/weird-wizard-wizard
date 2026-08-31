use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;
use crate::db::{ancestries::RawAncestry, paths::CharacterPath};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct RawCharacter {
    pub id: i64,
    name: String,
    level: i64,
    strength: i64,
    agility: i64,
    intellect: i64,
    will: i64,
    ancestry_id: i64,
    novice_path_id: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GetFullCharacter {
    pub id: i64,
    name: String,
    level: i64,
    strength: i64,
    agility: i64,
    intellect: i64,
    will: i64,
    ancestry: RawAncestry,
    novice_path: CharacterPath,
    expert_path: Option<CharacterPath>,
    master_path: Option<CharacterPath>,
    choices: (),
}