use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{
    import::{ProfessionCategoryRow, ProfessionRow},
    WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct Profession {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub category: String,
}

impl Profession {
    pub async fn insert_all(tx: &mut SqliteConnection, rows: &Vec<ProfessionRow>) -> WWResult<()> {
        for row in rows {
            sqlx::query!(
                "INSERT INTO professions (name, description, category) VALUES (?, ?, ?)",
                row.name,
                row.description,
                row.category
            )
            .execute(&mut *tx)
            .await?;
        }
        Ok(())
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct ProfessionCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}

impl ProfessionCategory {
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
            .await?;
        }
        Ok(())
    }
}
