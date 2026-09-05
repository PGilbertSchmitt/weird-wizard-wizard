use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::{
        etc::TalentRestore,
        info_tables::{self, FullInfoTable},
        option_blocks::{self, FullOptionBlock},
    },
    import::{MagicTalentRow, NameToId},
    WWError, WWResult,
};

// An intermediary struct for one fewer query
#[derive(Serialize, Deserialize)]
struct RawMagicTalent {
    id: i64,
    tradition_id: i64,
    tradition_name: String,
    name: String,
    description: String,
    charges: Option<String>,
    restore: TalentRestore,
    activate: String,
    info_table_id: Option<i64>,
    option_block_id: Option<i64>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "magic.ts")]
pub enum MagicTalentCharges {
    None,
    One,
    OneTwoThree,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "magic.ts")]
pub struct FullMagicTalent {
    id: i64,
    tradition_id: i64,
    tradition_name: String,
    name: String,
    description: String,
    charges: MagicTalentCharges,
    restore: TalentRestore,
    activate: String,
    info_table: Option<FullInfoTable>,
    option_block: Option<FullOptionBlock>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    magic_talents: &Vec<MagicTalentRow>,
    trad_map: &NameToId,
    table_map: &NameToId,
    option_map: &NameToId,
) -> WWResult<()> {
    for row in magic_talents {
        let tradition_id = trad_map.get_id(&row.tradition)?;
        let table_id = table_map.get_id_from_opt(&row.table)?;
        let options_id = option_map.get_id_from_opt(&row.options)?;

        sqlx::query!(
            "INSERT INTO magic_talents (
                tradition_id,
                name,
                description,
                charges,
                restore,
                activate,
                info_table_id,
                option_block_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            tradition_id,
            row.talent_name,
            row.description,
            row.charges,
            row.restore,
            row.activate,
            table_id,
            options_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding magic talent {}: {}",
                row.talent_name, e
            ))
        })?;
    }

    Ok(())
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullMagicTalent> {
    let talent = sqlx::query_as!(
        RawMagicTalent,
        "SELECT mt.*, t.name as tradition_name
        FROM magic_talents mt
        JOIN traditions t ON t.id = mt.tradition_id
        WHERE mt.id = ?",
        id
    )
    .fetch_one(db)
    .await?;

    let info_table = info_tables::get_from_opt(db, talent.info_table_id).await?;
    let option_block = option_blocks::get_from_opt(db, talent.option_block_id).await?;

    let charges = match talent.charges.as_deref() {
        Some("1") => MagicTalentCharges::One,
        Some("123") => MagicTalentCharges::OneTwoThree,
        _ => MagicTalentCharges::None,
    };

    Ok(FullMagicTalent {
        id,
        tradition_id: talent.tradition_id,
        tradition_name: talent.tradition_name,
        name: talent.name,
        description: talent.description,
        charges: charges,
        restore: talent.restore,
        activate: talent.activate,
        info_table: info_table,
        option_block: option_block,
    })
}

pub async fn get_for_tradition(db: &Pool<Sqlite>, tradition_id: i64) -> WWResult<Vec<FullMagicTalent>> {
    let talent_ids = sqlx::query_scalar!(
        "SELECT id FROM magic_talents WHERE tradition_id = ?",
        tradition_id,
    ).fetch_all(db).await?;

    let talents = futures::stream::iter(talent_ids)
        .map(|id| async move { get(db, id).await })
        .buffered(10)
        .try_collect()
        .await?;

    Ok(talents)
}
