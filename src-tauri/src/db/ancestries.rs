use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    WWError, WWResult, db::{
        etc, immunities::Immunity, languages::{self, Language}, path_talents::{self, FullPathTalent}, speed_traits::FullSpeedTrait,
    }, import::{AncestryRow, NamePairToId, NameToId, pipe_separate},
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
struct RawAncestry {
    pub id: i64,
    name: String,
    descriptor: Option<String>,
    size: etc::Size,
    speed: i64,
    add_health: Option<i64>,
    add_nat_def: Option<i64>,
}
#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct FullAncestry {
    pub id: i64,
    name: String,
    descriptor: Option<String>,
    size: etc::Size,
    speed: i64,
    add_health: Option<i64>,
    add_nat_def: Option<i64>,
    languages: Vec<Language>,
    immunities: Vec<Immunity>,
    speed_traits: Vec<FullSpeedTrait>,
    senses: Vec<AncestrySense>,
    talents: Vec<FullPathTalent>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct AncestrySense {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
    pub amount: Option<String>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    rows: &Vec<AncestryRow>,
    language_map: &NameToId,
    speed_trait_map: &NameToId,
    sense_map: &NameToId,
    immunity_map: &NameToId,
    path_talent_map: &NamePairToId,
) -> WWResult<NameToId> {
    let mut ancestry_map = NameToId::new("ancestry");

    for row in rows {
        let label = row.ancestry.clone();
        let record = sqlx::query!(
            "INSERT INTO ancestries (
                name,
                descriptor,
                size,
                speed,
                add_health,
                add_nat_def
            ) VALUES (?, ?, ?, ?, ?, ?)",
            row.ancestry,
            row.descriptor,
            row.base_size,
            row.base_speed,
            row.add_health,
            row.add_nat_def
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding ancestry row {}: {}",
                row.ancestry, e
            ))
        })?;

        let ancestry_id = record.last_insert_rowid();
        ancestry_map.insert(label.clone(), ancestry_id);

        for language in pipe_separate(&row.languages) {
            let language_id = language_map.get_id(&language)?;
            sqlx::query!(
                "INSERT INTO ancestry_languages (ancestry_id, language_id) VALUES (?, ?)",
                ancestry_id,
                language_id
            )
            .execute(&mut *tx)
            .await?;
        }

        for speed_trait in pipe_separate(&row.speed_traits) {
            let mut parts = speed_trait.split("=");
            if let Some(speed_trait_name) = parts.next() {
                let speed_trait_id = speed_trait_map.get_id(&speed_trait_name.to_string())?;
                let distance = parts.next();
                sqlx::query!(
                    "INSERT INTO ancestry_speed_traits (ancestry_id, speed_trait_id, amount) VALUES (?, ?, ?)",
                    ancestry_id,
                    speed_trait_id,
                    distance
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        for sense in pipe_separate(&row.senses) {
            // No official Ancestry has a sense requiring a distance, but it's nice to support
            // custom entries, no?
            let mut parts = sense.split("=");
            if let Some(sense_name) = parts.next() {
                let sense_id = sense_map.get_id(&sense_name.to_string())?;
                let distance = parts.next();
                sqlx::query!(
                    "INSERT INTO ancestry_senses (ancestry_id, sense_id, amount) VALUES (?, ?, ?)",
                    ancestry_id,
                    sense_id,
                    distance
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        for immunity in pipe_separate(&row.immunities) {
            let immunity_id = immunity_map.get_id(&immunity)?;
            sqlx::query!(
                "INSERT INTO ancestry_immunities (ancestry_id, immunity_id) VALUES (?, ?)",
                ancestry_id,
                immunity_id
            )
            .execute(&mut *tx)
            .await?;
        }

        for talent in pipe_separate(&row.traits) {
            let path_talent_id = path_talent_map.get_id(&(talent, label.clone()))?;
            sqlx::query!(
                "INSERT INTO ancestry_talents (ancestry_id, path_talent_id) VALUES (?, ?)",
                ancestry_id,
                path_talent_id,
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    Ok(ancestry_map)
}

async fn get_raw_ancestry(db: &Pool<Sqlite>, id: i64) -> WWResult<RawAncestry> {
    let record = sqlx::query_as!(RawAncestry, "SELECT * FROM ancestries WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    Ok(record)
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullAncestry> {
    let (ancestry, languages, speed_traits, senses, immunities, talents) = futures::join!(
        get_raw_ancestry(db, id),
        languages::get_for_ancestry(db, id),
        sqlx::query_as!(
            FullSpeedTrait,
            "SELECT st.*, ast.amount
            FROM speed_traits as st
            JOIN ancestry_speed_traits ast ON ast.speed_trait_id = st.id
            WHERE ast.ancestry_id = ?",
            id
        )
        .fetch_all(db),
        sqlx::query_as!(
            AncestrySense,
            "SELECT s.*, a_s.amount FROM senses as s
            JOIN ancestry_senses a_s ON a_s.sense_id = s.id
            WHERE a_s.ancestry_id = ?",
            id
        )
        .fetch_all(db),
        sqlx::query_as!(
            Immunity,
            "SELECT i.* FROM immunities as i
            JOIN ancestry_immunities ai ON ai.immunity_id = i.id
            WHERE ai.ancestry_id = ?",
            id
        )
        .fetch_all(db),
        path_talents::get_for_ancestry(db, id),
    );
    let ancestry = ancestry?;
    let languages = languages?;
    let speed_traits = speed_traits?;
    let senses = senses?;
    let immunities = immunities?;
    let talents = talents?;

    Ok(FullAncestry {
        id,
        name: ancestry.name,
        descriptor: ancestry.descriptor,
        size: ancestry.size,
        speed: ancestry.speed,
        add_health: ancestry.add_health,
        add_nat_def: ancestry.add_nat_def,
        languages,
        speed_traits,
        senses,
        immunities,
        talents,
    })
}

// TODO: Target for optimization
pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<FullAncestry>> {
    let ids: Vec<i64> = sqlx::query_scalar!("SELECT id FROM ancestries")
        .fetch_all(db)
        .await?;

    let ancestries: Vec<FullAncestry> = futures::stream::iter(ids)
        .map(|id| async move { get(db, id).await })
        .buffered(10)
        .try_collect()
        .await?;

    Ok(ancestries)
}
