use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    WWError, WWResult, import::{LanguageRow, NameToId, is_affirmative}, util::db_boolean,
};

#[derive(Serialize, Deserialize)]
struct RawLanguage {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub secret: Option<String>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct Language {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub secret: bool,
}

pub async fn insert_all(tx: &mut SqliteConnection, rows: &Vec<LanguageRow>) -> WWResult<NameToId> {
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
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding language {}: {}",
                label, e
            ))
        })?;

        name_to_id.insert(label, record.last_insert_rowid());
    }

    Ok(name_to_id)
}

pub async fn get_for_ancestry(db: &Pool<Sqlite>, ancestry_id: i64) -> WWResult<Vec<Language>> {
    let languages = sqlx::query_as!(
        RawLanguage,
        "SELECT l.* FROM languages as l
        JOIN ancestry_languages a_l ON a_l.language_id = l.id
        WHERE a_l.ancestry_id = ?",
        ancestry_id,
    ).fetch_all(db).await?;

    Ok(languages.into_iter().map(|l| Language {
        id: l.id,
        name: l.name,
        description: l.description,
        secret: db_boolean(l.secret),
    }).collect())
}
