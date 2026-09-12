use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use ts_rs::TS;

use crate::{
    db::{ancestries::FullAncestry, paths::FullPath},
    WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct CreateCharacter {
    name: String,
    profession_id: i64,
    ancestry_id: i64,
    novice_path_id: i64,
    strength: i64,
    agility: i64,
    intellect: i64,
    will: i64,
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
    )
    .fetch_all(db)
    .await?;

    Ok(rows)
}

pub async fn create_character(db: &Pool<Sqlite>, character_info: CreateCharacter) -> WWResult<i64> {
    // Initial character max health is determined by the chosen ancestry and the first level of the chosen Novice path.
    let (ancestry_health, path_health) = futures::join!(
        sqlx::query_scalar!(
            "SELECT add_health FROM ancestries WHERE id = ?",
            character_info.ancestry_id
        )
        .fetch_one(db),
        sqlx::query_scalar!(
            "SELECT add_health FROM levels WHERE level = 1 AND path_id = ?",
            character_info.novice_path_id
        )
        .fetch_one(db),
    );

    let init_health = path_health? + ancestry_health?.unwrap_or(0);

    let record = sqlx::query!(
        "INSERT INTO characters (
            name,
            level,
            health,
            damage,
            strength,
            agility,
            intellect,
            will,
            profession_id,
            ancestry_id,
            novice_path_id
        ) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        character_info.name,
        1,
        init_health,
        init_health,
        character_info.strength,
        character_info.agility,
        character_info.intellect,
        character_info.will,
        character_info.profession_id,
        character_info.ancestry_id,
        character_info.novice_path_id
    )
    .execute(db)
    .await?;

    Ok(record.last_insert_rowid())
}
