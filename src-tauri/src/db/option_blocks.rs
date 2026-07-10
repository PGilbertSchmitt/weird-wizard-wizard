use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    import::{NameToId, OptionRow},
    WWResult,
};

// #[ts(export, export_to = "option_blocks.ts")]
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionBlock {
    pub id: i64,
    pub name: String,
}
// option_block_id INTEGER REFERENCES option_blocks(id)
impl OptionBlock {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        options: &Vec<OptionRow>,
    ) -> WWResult<NameToId> {
        let mut option_map = NameToId::new("option_block");

        for row in options {
            let option_id = match option_map.get_id(&row.options_id) {
                Err(_) => {
                    let label = row.options_id.clone();
                    let record = sqlx::query!(
                        "INSERT INTO option_blocks (name) VALUES (?)",
                        row.options_id,
                    )
                    .execute(&mut *tx)
                    .await?;
                    let id = record.last_insert_rowid();
                    option_map.insert(label, id);
                    id
                }
                Ok(id) => id,
            };

            sqlx::query!(
                "INSERT INTO option_block_rows (option_block_id, value) VALUES (?, ?)",
                option_id,
                row.description,
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(option_map)
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "option_blocks.ts")]
pub struct FullOptionBlock {
    pub id: i64,
    pub name: String,
    pub entries: Vec<String>,
}

impl FullOptionBlock {
    pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<Self> {
        let name = sqlx::query_scalar!("SELECT name FROM option_blocks WHERE id = ?", id)
            .fetch_one(db)
            .await?;

        let entries: Vec<String> = sqlx::query_scalar!(
            "SELECT value FROM option_block_rows WHERE option_block_id = ?",
            id
        )
        .fetch_all(db)
        .await?;

        Ok(FullOptionBlock { id, name, entries })
    }

    pub async fn get_from_opt(db: &Pool<Sqlite>, id: Option<i64>) -> WWResult<Option<Self>> {
        Ok(if let Some(id_unwrapped) = id {
            Some(Self::get(db, id_unwrapped).await?)
        } else {
            None
        })
    }
}
