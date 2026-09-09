use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{import::ProfessionRow, WWError, WWResult};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct Profession {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub category: String,
}

pub async fn insert_all(tx: &mut SqliteConnection, rows: &Vec<ProfessionRow>) -> WWResult<()> {
    for row in rows {
        sqlx::query!(
            "INSERT INTO professions (name, description, category) VALUES (?, ?, ?)",
            row.name,
            row.description,
            row.category
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding profession {}: {}",
                row.name, e
            ))
        })?;
    }
    Ok(())
}

pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<Profession>> {
    let professions = sqlx::query_as!(Profession, "SELECT * FROM professions")
        .fetch_all(db)
        .await?;

    return Ok(professions);
}
