use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::{etc, immunities::Immunity, languages::Language},
    import::{pipe_separate, AncestryRow, NameToId},
    WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct Ancestry {
    pub id: i64,
    name: String,
    descriptor: Option<String>,
    size: etc::Size,
    speed: i64,
    add_health: Option<i64>,
    add_nat_def: Option<i64>,
}

impl Ancestry {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        rows: &Vec<AncestryRow>,
        language_map: &NameToId,
        speed_trait_map: &NameToId,
        sense_map: &NameToId,
        immunity_map: &NameToId,
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
            .await?;

            let ancestry_id = record.last_insert_rowid();
            ancestry_map.insert(label, ancestry_id);

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
                    let speed_trait_id = speed_trait_map.get_id(speed_trait_name)?;
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
                    let sense_id = sense_map.get_id(sense_name)?;
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
        }

        Ok(ancestry_map)
    }

    pub async fn get_ancestry(db: &Pool<Sqlite>, id: i64) -> WWResult<Ancestry> {
        let record = sqlx::query_as!(Ancestry, "SELECT * FROM ancestries WHERE id = ?", id)
            .fetch_one(db)
            .await?;

        Ok(record)
    }

    pub async fn get_full_ancestry(db: &Pool<Sqlite>, id: i64) -> WWResult<FullAncestry> {
        let (ancestry, languages, speed_traits, senses, immunities) = futures::join!(
            Self::get_ancestry(db, id),
            // These could probably be better if I made a macro
            sqlx::query_as!(
                Language,
                "SELECT l.* FROM languages as l
                JOIN ancestry_languages a_l ON a_l.language_id = l.id
                JOIN ancestries a ON a.id = a_l.ancestry_id
                WHERE a.id = ?",
                id
            )
            .fetch_all(db),
            sqlx::query_as!(
                AncestrySpeedTrait,
                "SELECT l.*, a_st.amount FROM speed_traits as l
                JOIN ancestry_speed_traits a_st ON a_st.speed_trait_id = l.id
                JOIN ancestries a ON a.id = a_st.ancestry_id
                WHERE a.id = ?",
                id
            )
            .fetch_all(db),
            sqlx::query_as!(
                AncestrySense,
                "SELECT l.*, a_s.amount FROM senses as l
                JOIN ancestry_senses a_s ON a_s.sense_id = l.id
                JOIN ancestries a ON a.id = a_s.ancestry_id
                WHERE a.id = ?",
                id
            )
            .fetch_all(db),
            sqlx::query_as!(
                Immunity,
                "SELECT l.* FROM immunities as l
                JOIN ancestry_immunities a_i ON a_i.immunity_id = l.id
                JOIN ancestries a ON a.id = a_i.ancestry_id
                WHERE a.id = ?",
                id
            )
            .fetch_all(db),
        );
        let ancestry = ancestry?;
        let languages = languages?;
        let speed_traits = speed_traits?;
        let senses = senses?;
        let immunities = immunities?;

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
        })
    }
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
    speed_traits: Vec<AncestrySpeedTrait>,
    senses: Vec<AncestrySense>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct AncestrySpeedTrait {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
    pub amount: Option<String>,
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
