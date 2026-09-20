use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Pool, Row, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    import::{NameToId, SpeedTraitRow},
    WWError, WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct RawSpeedTrait {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for RawSpeedTrait {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            unit: row.try_get("unit")?,
        })
    }
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "other_info.ts")]
pub struct FullSpeedTrait {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
    pub amount: Option<String>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    rows: &Vec<SpeedTraitRow>,
) -> WWResult<NameToId> {
    let mut name_to_id = NameToId::new("speed trait");

    for row in rows {
        let label = row.name.clone();
        let record = sqlx::query!(
            "INSERT INTO speed_traits (name, description, unit) VALUES (?, ?, ?)",
            row.name,
            row.description,
            row.unit,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding speed trait {}: {}",
                row.name, e
            ))
        })?;

        name_to_id.insert(label, record.last_insert_rowid());
    }

    Ok(name_to_id)
}

pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<RawSpeedTrait>> {
    let speed_traits = sqlx::query_as!(RawSpeedTrait, "SELECT * FROM speed_traits",)
        .fetch_all(db)
        .await?;
    Ok(speed_traits)
}
