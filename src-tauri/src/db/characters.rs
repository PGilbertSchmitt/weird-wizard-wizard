use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{WWResult, db::{ancestries::FullAncestry, paths::FullPath}};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawCharacter {
    id: i64,
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
pub struct FullCharacter {
    id: i64,
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

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct CharacterIndexItem {
    id: i64,
    name: String,
    level: i64,
    ancestry: String,
    novice_path: String,
    expert_path: Option<String>,
    master_path: Option<String>,
}

pub async fn get_index(db: &Pool<Sqlite>) -> WWResult<Vec<CharacterIndexItem>> {
    let rows = sqlx::query_as!(
        CharacterIndexItem,
        "SELECT
            c.id,
            c.name,
            c.level,
            a.name AS ancestry,
            np.name AS novice_path,
            ep.name AS expert_path,
            mp.name AS master_path
        FROM characters c
        JOIN ancestries a ON a.id = c.ancestry_id
        JOIN paths np ON np.id = c.novice_path_id
        LEFT JOIN paths ep ON ep.id = c.expert_path_id
        LEFT JOIN paths mp ON mp.id = c.master_path_id"
    ).fetch_all(db).await?;

    Ok(rows)
}
