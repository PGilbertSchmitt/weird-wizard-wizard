use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{WWResult, import::{NameToId, SenseRow}};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to="other_info.ts")]
pub struct Sense {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
}

impl Sense {
    pub async fn insert_all(tx: &mut SqliteConnection, rows: &Vec<SenseRow>) -> WWResult<NameToId> {
        let mut name_to_id = NameToId::new();
        // Linear execution is probably fine for now
        for row in rows {
            let label = row.name.clone();
            let record = sqlx::query!(
                "INSERT INTO senses (name, description, unit) VALUES (?, ?, ?)",
                row.name,
                row.description,
                row.unit,
            )
                .execute(&mut *tx)
                .await?;

            name_to_id.insert(label, record.last_insert_rowid());
        }
        
        Ok(name_to_id)
    }
}
