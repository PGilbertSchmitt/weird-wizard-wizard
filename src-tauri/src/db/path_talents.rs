use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};

use crate::{WWError, WWResult, db::etc::TalentRestore, import::{NamePairToId, NameToId, PathTalentRow, is_affirmative}};

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
            ).execute(&mut *tx).await.map_err(|e|
                WWError::Generic(format!("Encountered error while seeding path talents {}: {}", row.name, e))
            )?;

            talent_map.insert((row.name.clone(), source.to_string()), record.last_insert_rowid());
        }
    }

    Ok(talent_map)
}

#[derive(Debug, Serialize, Deserialize)]
struct PathTalent {
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

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<PathTalent> {
    let talent = sqlx::query_as!(
        PathTalent,
        "SELECT * FROM path_talents WHERE id = ?",
        id
    ).fetch_one(db).await?;
    Ok(talent)
}
