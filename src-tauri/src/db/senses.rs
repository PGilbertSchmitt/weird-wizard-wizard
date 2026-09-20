use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    import::{NameToId, SenseRow},
    WWError, WWResult,
};

#[derive(Debug)]
pub struct RawSense {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "other_info.ts")]
pub struct FullSense {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
    pub amount: Option<String>,
}

pub async fn insert_all(tx: &mut SqliteConnection, rows: &Vec<SenseRow>) -> WWResult<NameToId> {
    let mut name_to_id = NameToId::new("sense");

    for row in rows {
        let label = row.name.clone();
        let record = sqlx::query!(
            "INSERT INTO senses (name, description, unit) VALUES (?, ?, ?)",
            row.name,
            row.description,
            row.unit,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding sense {}: {}",
                row.name, e
            ))
        })?;

        name_to_id.insert(label, record.last_insert_rowid());
    }

    Ok(name_to_id)
}

pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<RawSense>> {
    let senses = sqlx::query_as!(RawSense, "SELECT * FROM senses",)
        .fetch_all(db)
        .await?;
    Ok(senses)
}
