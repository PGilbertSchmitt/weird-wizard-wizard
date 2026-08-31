use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{
    WWError, WWResult, import::{NameToId, SpeedTraitRow},
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct SpeedTrait {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub unit: Option<String>,
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
        .await.map_err(|e|
            WWError::Generic(format!("Encountered error while seeding speed trait {}: {}", row.name, e))
        )?;

        name_to_id.insert(label, record.last_insert_rowid());
    }

    Ok(name_to_id)
}
