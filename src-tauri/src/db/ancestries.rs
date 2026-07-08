use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{WWResult, db::etc, import::{AncestryRow, NameToId, pipe_separate}, result::opt_to_wwresult};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct Ancestry {
    pub id:      u32,
    name:        String,
    descriptor:  Option<String>,
    size:        etc::Size,
    speed:       i32,
    add_health:  i32,
    add_nat_def: i32
}

impl Ancestry {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        rows: &Vec<AncestryRow>,
        language_map: NameToId,
        speed_trait_map: NameToId,
        sense_map: NameToId,
        immunity_map: NameToId,
    ) -> WWResult<()> {
        // Linear execution is probably fine for now
        for row in rows {
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

            for language in pipe_separate(&row.languages) {
                let language_id = opt_to_wwresult(
                    language_map.get(&language),
                    format!("Error while creating '{}' ancestry: language '{}' not found in source file", row.ancestry, &language)
                )?;
                sqlx::query!(
                    "INSERT INTO ancestry_languages (
                        ancestry_id,
                        language_id
                    ) VALUES (?, ?)",
                    ancestry_id,
                    language_id
                )
                    .execute(&mut *tx)
                    .await?;
            }

            for speed_trait in pipe_separate(&row.speed_traits) {
                let mut parts = speed_trait.split("=");
                if let Some(speed_trait_name) = parts.next() {
                    let speed_trait_id = opt_to_wwresult(
                        speed_trait_map.get(speed_trait_name),
                        format!("Error while creating '{}' ancestry: speed trait '{}' not found in source file", row.ancestry, speed_trait_name)
                    )?;
                    let distance = parts.next();
                    sqlx::query!(
                        "INSERT INTO ancestry_speed_traits (
                            ancestry_id,
                            speed_trait_id,
                            amount
                        ) VALUES (?, ?, ?)",
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
                    let sense_id = opt_to_wwresult(
                        sense_map.get(sense_name),
                        format!("Error while creating '{}' ancestry: sense '{}' not found in source file", row.ancestry, sense_name)
                    )?;
                    let distance = parts.next();
                    sqlx::query!(
                        "INSERT INTO ancestry_senses (
                            ancestry_id,
                            sense_id,
                            amount
                        ) VALUES (?, ?, ?)",
                        ancestry_id,
                        sense_id,
                        distance
                    )
                        .execute(&mut *tx)
                        .await?;
                }
            }

            for immunity in pipe_separate(&row.immunities) {
                let immunity_id = opt_to_wwresult(
                    immunity_map.get(&immunity),
                    format!("Error while creating '{}' ancestry: immunity '{}' not found in source file", row.ancestry, &immunity)
                )?;
                sqlx::query!(
                    "INSERT INTO ancestry_immunities (
                        ancestry_id,
                        immunity_id
                    ) VALUES (?, ?)",
                    ancestry_id,
                    immunity_id
                )
                    .execute(&mut *tx)
                    .await?;
            }
        }

        Ok(())
    }
}
