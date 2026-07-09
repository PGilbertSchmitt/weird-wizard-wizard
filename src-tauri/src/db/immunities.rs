use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{import::NameToId, WWResult};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct Immunity {
    pub id: i64,
    pub name: String,
}

impl Immunity {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        immunities: &HashSet<String>,
    ) -> WWResult<NameToId> {
        let mut name_to_id = NameToId::new();
        // Linear execution is probably fine for now
        for immunity in immunities {
            let label = immunity.clone();
            let record = sqlx::query!("INSERT INTO immunities (name) VALUES (?)", immunity,)
                .execute(&mut *tx)
                .await?;

            name_to_id.insert(label, record.last_insert_rowid());
        }

        Ok(name_to_id)
    }
}
