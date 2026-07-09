use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;
use ts_rs::TS;

use crate::{import::OptionRow, WWResult};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "option_blocks.ts")]
pub struct OptionBlock {
    pub label: String,
    pub values: Vec<String>,
}

impl OptionBlock {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        options: &Vec<OptionRow>,
    ) -> WWResult<HashSet<String>> {
        let mut option_labels = HashSet::new();

        for option in options {
            let label = option.options_id.clone();
            option_labels.insert(label.clone());
            sqlx::query!(
                "INSERT INTO option_blocks (label, value) VALUES (?, ?)",
                option.options_id,
                option.description
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(option_labels)
    }
}
