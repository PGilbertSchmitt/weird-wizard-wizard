use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};

use crate::{
    import::{ChoiceSelectionRow, NameToId},
    mod_dsl::parser::parse_mods,
    modifiers::{FullModifier, HasModifiers},
    WWError, WWResult,
};

#[derive(Debug)]
pub struct RawChoice {
    id: i64,
    label: Option<String>,
    description: String,
    mod_str: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullChoice {
    id: i64,
    label: Option<String>,
    description: String,
    modifiers: Vec<FullModifier>,
}

impl HasModifiers for FullChoice {
    fn modifiers(&self) -> Vec<FullModifier> {
        self.modifiers.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceTable {
    pub id: i64,
    pub name: String,
    pub choices: Vec<FullChoice>,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    choice_selections: &Vec<ChoiceSelectionRow>,
) -> WWResult<()> {
    let mut choice_map = NameToId::new("choice_selection");

    for row in choice_selections {
        let choice_id = match choice_map.get_id(&row.choice_name) {
            Err(_) => {
                let name = row.choice_name.clone();
                let record = sqlx::query!("INSERT INTO choice_tables (name) VALUES (?)", name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        WWError::Generic(format!(
                            "Encountered error while seeding choice selection row {}: {}",
                            row.choice_name, e
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
            row.choice_header,
            row.choice_text,
            row.mod_str,
        ).execute(&mut *tx).await.map_err(|e|
            WWError::Generic(format!("Encountered error while seeding choice selection row {}, key {}: {}", row.choice_header, row.choice_name, e))
        )?;
    }

    Ok(())
}

pub async fn _get_choice_table(db: &Pool<Sqlite>, id: i64) -> WWResult<ChoiceTable> {
    let name = sqlx::query_scalar!("SELECT name FROM choice_tables WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    let choices = sqlx::query_as!(
        RawChoice,
        "SELECT id, label, description, mod_str FROM choice_selections WHERE choice_table_id = ?",
        id
    )
    .fetch_all(db)
    .await?;

    let mut full_choices = Vec::with_capacity(choices.len());
    for raw_choice in choices {
        let full_choice = convert_choice(raw_choice)?;
        full_choices.push(full_choice)
    }

    Ok(ChoiceTable {
        id,
        name,
        choices: full_choices,
    })
}

async fn get_full_choice_selection(db: &Pool<Sqlite>, id: i64) -> WWResult<FullChoice> {
    let raw_choice = sqlx::query_as!(
        RawChoice,
        "SELECT id, label, description, mod_str FROM choice_selections WHERE id = ?",
        id,
    )
    .fetch_one(db)
    .await?;

    convert_choice(raw_choice)
}

fn convert_choice(raw_choice: RawChoice) -> WWResult<FullChoice> {
    let choice_path_str = format!(
        "Choice;{}",
        raw_choice
            .label
            .clone()
            .unwrap_or(raw_choice.id.to_string())
    );
    let modifiers = raw_choice
        .mod_str
        .map(|mod_str| {
            parse_mods(&mod_str).map(|base_modifiers| {
                base_modifiers
                    .into_iter()
                    .map(|mod_details| FullModifier {
                        path_str: choice_path_str.clone(),
                        mod_details,
                    })
                    .collect()
            })
        })
        .unwrap_or(Ok(Vec::new()))?;

    Ok(FullChoice {
        id: raw_choice.id,
        label: raw_choice.label,
        description: raw_choice.description,
        modifiers,
    })
}

// This needs to build mods the same way
pub async fn get_for_choice_ids(db: &Pool<Sqlite>, ids: Vec<i64>) -> WWResult<Vec<FullChoice>> {
    let selections: Vec<FullChoice> = futures::stream::iter(ids)
        .map(|id| {
            let db = db.clone();
            async move { get_full_choice_selection(&db, id).await }
        })
        .buffered(100)
        .try_collect()
        .await?;

    Ok(selections)
}
