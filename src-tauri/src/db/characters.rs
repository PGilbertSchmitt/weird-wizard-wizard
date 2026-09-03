use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::db::{ancestries::FullAncestry, paths::FullPath};

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
    ancestry: FullAncestry,
    novice_path: FullPath,
    expert_path: Option<FullPath>,
    master_path: Option<FullPath>,
    choices: (),
}
