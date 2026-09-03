use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};

use crate::{
    import::{ChoiceSelectionRow, NameToId},
    WWError, WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    label: Option<String>,
    description: String,
    mod_str: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceTable {
    pub id: i64,
    pub name: String,
    pub choices: Vec<Choice>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    choice_selections: &Vec<ChoiceSelectionRow>,
) -> WWResult<()> {
    let mut choice_map = NameToId::new("choice_selection");

    for row in choice_selections {
        let choice_id = match choice_map.get_id(&row.choice_header) {
            Err(_) => {
                let name = row.choice_header.clone();
                let record = sqlx::query!("INSERT INTO choice_tables (name) VALUES (?)", name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        WWError::Generic(format!(
                            "Encountered error while seeding choice selection row {}: {}",
                            row.choice_header, e
                        ))
                    })?;

                let id = record.last_insert_rowid();
                choice_map.insert(name, id);
                id
            }
            Ok(id) => id,
        };

        sqlx::query!(
            "INSERT INTO choice_selections (choice_table_id, label, description, mod_str) VALUES (?, ?, ?, ?)",
            choice_id,
            row.choice_name,
            row.choice_text,
            row.mod_str,
        ).execute(&mut *tx).await.map_err(|e|
            WWError::Generic(format!("Encountered error while seeding choice selection row {}, key {}: {}", row.choice_header, row.choice_name, e))
        )?;
    }

    Ok(())
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<ChoiceTable> {
    let name = sqlx::query_scalar!("SELECT name FROM choice_tables WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    let choices = sqlx::query_as!(
        Choice,
        "SELECT label, description, mod_str FROM choice_selections WHERE choice_table_id = ?",
        id
    )
    .fetch_all(db)
    .await?;

    Ok(ChoiceTable { id, name, choices })
}
