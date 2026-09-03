use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::{
        etc,
        path_talents::{self, FullPathTalent},
        speed_traits::FullSpeedTrait,
    },
    import::{pipe_separate, NamePairToId, NameToId, PathLevelRow},
    WWError, WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawLevel {
    id: i64,
    path_id: i64,
    level: i64,
    add_health: Option<i64>,
    add_nat_def: Option<i64>,
    add_arm_def: Option<i64>,
    add_bonus_dmg: Option<i64>,
    add_speed: Option<i64>,
    trad_choices: Option<i64>,
    lang_choices: Option<i64>,
    novice_spells: Option<i64>,
    expert_spells: Option<i64>,
    master_spells: Option<i64>,
    size: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct LevelTradition {
    name: String,
    blurb: String,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct FullLevel {
    id: i64,
    path_id: i64,
    level: i64,
    add_health: i64,
    add_nat_def: i64,
    add_arm_def: i64,
    add_bonus_dmg: i64,
    add_speed: i64,
    trad_choices: i64,
    lang_choices: i64,
    novice_spells: i64,
    expert_spells: i64,
    master_spells: i64,
    size: Option<etc::Size>,
    languages: Vec<String>,
    path_talents: Vec<FullPathTalent>,
    speed_traits: Vec<FullSpeedTrait>,
    traditions: Vec<LevelTradition>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    levels: &Vec<PathLevelRow>,
    path_map: &NameToId,
    trad_map: &NameToId,
    language_map: &NameToId,
    speed_trait_map: &NameToId,
    path_talent_map: &NamePairToId,
) -> WWResult<()> {
    for row in levels {
        let path_id = path_map.get_id(&row.path)?;
        let record = sqlx::query!(
            "INSERT INTO levels (
                path_id,
                level,
                add_health,
                add_nat_def,
                add_arm_def,
                add_bonus_dmg,
                add_speed,
                trad_choices,
                lang_choices,
                novice_spells,
                expert_spells,
                master_spells,
                size
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            path_id,
            row.level,
            row.health,
            row.nat_def,
            row.armed_def,
            row.bonus_dmg,
            row.speed,
            row.trad_choices,
            row.lang_choices,
            row.novice_spells,
            row.expert_spells,
            row.master_spells,
            row.size
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding {} level {}: {}",
                path_id, row.level, e
            ))
        })?;
        let level_id = record.last_insert_rowid();

        for tradition in pipe_separate(&row.traditions) {
            let trad_id = trad_map.get_id(&tradition)?;
            sqlx::query!(
                "INSERT INTO level_traditions (level_id, tradition_id) VALUES (?, ?)",
                level_id,
                trad_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        for language in pipe_separate(&row.languages) {
            let lang_id = language_map.get_id(&language)?;
            sqlx::query!(
                "INSERT INTO level_languages (level_id, language_id) VALUES (?, ?)",
                level_id,
                lang_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        for speed_trait in pipe_separate(&row.speed_traits) {
            let speed_trait_id = speed_trait_map.get_id(&speed_trait)?;
            sqlx::query!(
                "INSERT INTO level_speed_traits (level_id, speed_trait_id) VALUES (?, ?)",
                level_id,
                speed_trait_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        for talent in pipe_separate(&row.talents) {
            let path_talent_id = path_talent_map.get_id(&(talent, row.path.clone()))?;
            sqlx::query!(
                "INSERT INTO level_talents (level_id, path_talent_id) VALUES (?, ?)",
                level_id,
                path_talent_id,
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    Ok(())
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullLevel> {
    let raw_level = sqlx::query_as!(RawLevel, "SELECT * FROM levels WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    let (level_talents, level_languages, level_speed_traits, level_traditions) = futures::join!(
        path_talents::get_for_level(db, id),
        sqlx::query_scalar!(
            "SELECT l.name FROM languages as l
            JOIN level_languages ll ON ll.language_id = l.id
            WHERE ll.level_id = ?",
            id
        )
        .fetch_all(db),
        sqlx::query_as!(
            FullSpeedTrait,
            "SELECT st.*, lst.amount
            FROM speed_traits as st
            JOIN level_speed_traits lst ON lst.speed_trait_id = st.id
            WHERE lst.level_id = ?",
            id
        )
        .fetch_all(db),
        sqlx::query_as!(
            LevelTradition,
            "SELECT t.name, t.blurb
            FROM traditions t
            JOIN level_traditions lt ON lt.tradition_id = t.id
            WHERE lt.level_id = ?",
            id
        )
        .fetch_all(db),
    );

    Ok(FullLevel {
        id: raw_level.id,
        path_id: raw_level.path_id,
        level: raw_level.level,
        add_health: raw_level.add_health.unwrap_or(0),
        add_nat_def: raw_level.add_nat_def.unwrap_or(0),
        add_arm_def: raw_level.add_arm_def.unwrap_or(0),
        add_bonus_dmg: raw_level.add_bonus_dmg.unwrap_or(0),
        add_speed: raw_level.add_speed.unwrap_or(0),
        trad_choices: raw_level.trad_choices.unwrap_or(0),
        lang_choices: raw_level.lang_choices.unwrap_or(0),
        novice_spells: raw_level.novice_spells.unwrap_or(0),
        expert_spells: raw_level.expert_spells.unwrap_or(0),
        master_spells: raw_level.master_spells.unwrap_or(0),
        size: etc::Size::from_opt(raw_level.size),
        path_talents: level_talents?,
        languages: level_languages?,
        speed_traits: level_speed_traits?,
        traditions: level_traditions?,
    })
}

pub async fn get_for_path(db: &Pool<Sqlite>, path_id: i64) -> WWResult<Vec<FullLevel>> {
    let ids = sqlx::query_scalar!("SELECT id FROM levels WHERE path_id = ?", path_id)
        .fetch_all(db)
        .await?;

    let levels = futures::stream::iter(ids)
        .map(|id| async move { get(db, id).await })
        .buffered(100)
        .try_collect()
        .await?;

    Ok(levels)
}

// This takes ~250ms on average using naive queries. This is already on the bigger end of
// queries that this app would support (why would you query ALL levels w/ talents?), so
// this represents an okay upper bound. However, this can definitely be improved with
// join queries and row consolidation. Worth looking into.
// pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<FullLevel>> {
//     let ids: Vec<i64> = sqlx::query_scalar!(
//         "SELECT id FROM levels"
//     ).fetch_all(db).await?;

//     let levels = futures::stream::iter(ids)
//         .map(|id| async move { get(db, id).await })
//         .buffered(100)
//         .try_collect()
//         .await?;

//     Ok(levels)
// }
