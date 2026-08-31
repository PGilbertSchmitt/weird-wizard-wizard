use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{
    WWError, WWResult, import::ProfessionCategoryRow,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct ProfessionCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    rows: &Vec<ProfessionCategoryRow>,
) -> WWResult<()> {
    for row in rows {
        sqlx::query!(
            "INSERT INTO profession_categories (name, description) VALUES (?, ?)",
            row.name,
            row.description
        )
        .execute(&mut *tx)
        .await.map_err(|e|
            WWError::Generic(format!("Encountered error while seeding profession category {}: {}", row.name, e))
        )?;
    }
    Ok(())
}
