use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{
    WWError, WWResult, import::{LanguageRow, NameToId, is_affirmative},
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct Language {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub secret: bool,
}

impl Language {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        rows: &Vec<LanguageRow>,
    ) -> WWResult<NameToId> {
        let mut name_to_id = NameToId::new("language");

        for row in rows {
            let label = row.languages.clone();
            let secret = is_affirmative(row.secret.as_deref());
            let record = sqlx::query!(
                "INSERT INTO languages (name, description, secret) VALUES (?, ?, ?)",
                label,
                row.description,
                secret,
            )
            .execute(&mut *tx)
            .await.map_err(|e|
                WWError::Generic(format!("Encountered error while seeding language {}: {}", label, e))
            )?;

            name_to_id.insert(label, record.last_insert_rowid());
        }

        Ok(name_to_id)
    }
}
