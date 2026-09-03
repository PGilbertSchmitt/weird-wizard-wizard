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
    import::{is_affirmative, NamePairToId, NameToId, PathTalentRow},
    WWError, WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
struct RawPathTalent {
    id: i64,
    name: String,
    source: String,
    magical: bool,
    charges: Option<String>,
    restore: TalentRestore,
    activate: Option<String>,
    description: String,
    info_table_id: Option<i64>,
    option_block_id: Option<i64>,
    mod_str: Option<String>,
    cluster: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct FullPathTalent {
    id: i64,
    name: String,
    source: String,
    magical: bool,
    charges: Option<String>,
    restore: TalentRestore,
    activate: Option<String>,
    description: String,
    info_table_id: Option<FullInfoTable>,
    option_block_id: Option<FullOptionBlock>,
    mod_str: Option<String>,
    cluster: Option<String>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    path_talents: &Vec<PathTalentRow>,
    table_map: &NameToId,
    option_map: &NameToId,
) -> WWResult<NamePairToId> {
    let mut talent_map = NamePairToId::new("path_talent");

    for row in path_talents {
        let magical = is_affirmative(row.magical.as_deref());
        let table_id = table_map.get_id_from_opt(&row.table)?;
        let options_id = option_map.get_id_from_opt(&row.options)?;
        for source in row.source.split("|").map(|s| s.trim()) {
            let record = sqlx::query!(
                "INSERT INTO path_talents (
                    name,
                    source,
                    magical,
                    charges,
                    restore,
                    activate,
                    description,
                    info_table_id,
                    option_block_id,
                    mod_str,
                    cluster
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                row.name,
                source,
                magical,
                row.charges,
                row.restore,
                row.activate,
                row.description,
                table_id,
                options_id,
                row.mod_str,
                row.cluster,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                WWError::Generic(format!(
                    "Encountered error while seeding path talents {}: {}",
                    row.name, e
                ))
            })?;

            talent_map.insert(
                (row.name.clone(), source.to_string()),
                record.last_insert_rowid(),
            );
        }
    }

    Ok(talent_map)
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullPathTalent> {
    let raw_talent = sqlx::query_as!(RawPathTalent, "SELECT * FROM path_talents WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    let (info_table, option_block) = futures::join!(
        info_tables::get_from_opt(db, raw_talent.info_table_id),
        option_blocks::get_from_opt(db, raw_talent.option_block_id),
    );

    Ok(FullPathTalent {
        id,
        name: raw_talent.name,
        source: raw_talent.source,
        magical: raw_talent.magical,
        charges: raw_talent.charges,
        restore: raw_talent.restore,
        activate: raw_talent.activate,
        description: raw_talent.description,
        info_table_id: info_table?,
        option_block_id: option_block?,
        mod_str: raw_talent.mod_str,
        cluster: raw_talent.cluster,
    })
}

async fn get_from_ids(db: &Pool<Sqlite>, ids: Vec<i64>) -> WWResult<Vec<FullPathTalent>> {
    let path_talents: Vec<FullPathTalent> = futures::stream::iter(ids)
        .map(|id| async move { get(db, id).await })
        .buffered(10)
        .try_collect()
        .await?;

    Ok(path_talents)
}

pub async fn get_for_ancestry(
    db: &Pool<Sqlite>,
    ancestry_id: i64,
) -> WWResult<Vec<FullPathTalent>> {
    let ids: Vec<i64> = sqlx::query_scalar!(
        "SELECT path_talent_id FROM ancestry_talents WHERE ancestry_id = ?",
        ancestry_id
    )
    .fetch_all(db)
    .await?;
    get_from_ids(db, ids).await
}

pub async fn get_for_level(db: &Pool<Sqlite>, level_id: i64) -> WWResult<Vec<FullPathTalent>> {
    let ids: Vec<i64> = sqlx::query_scalar!(
        "SELECT path_talent_id FROM level_talents WHERE level_id = ?",
        level_id
    )
    .fetch_all(db)
    .await?;
    get_from_ids(db, ids).await
}
