use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Pool, Row, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    import::{is_affirmative, LanguageRow, NameToId},
    util::db_boolean,
    WWError, WWResult,
};

#[derive(Serialize, Deserialize)]
struct RawLanguage {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub secret: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for RawLanguage {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            secret: row.try_get("secret")?,
        })
    }
}

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
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

fn convert_language(raw: RawLanguage) -> Language {
    Language {
        id: raw.id,
        name: raw.name,
        description: raw.description,
        secret: db_boolean(raw.secret),
    }
}

fn convert_languages(raw: Vec<RawLanguage>) -> Vec<Language> {
    raw.into_iter()
        .map(convert_language)
        .collect()
}

pub async fn get_for_ancestry(db: &Pool<Sqlite>, ancestry_id: i64) -> WWResult<Vec<Language>> {
    let languages = sqlx::query_as!(
        RawLanguage,
        "SELECT l.* FROM languages as l
        JOIN ancestry_languages a_l ON a_l.language_id = l.id
        WHERE a_l.ancestry_id = ?",
        ancestry_id,
    )
    .fetch_all(db)
    .await?;

    Ok(convert_languages(languages))
}

pub async fn get_for_level(db: &Pool<Sqlite>, level_id: i64) -> WWResult<Vec<Language>> {
    let languages = sqlx::query_as!(
        RawLanguage,
        "SELECT l.* FROM languages as l
        JOIN level_languages l_l ON l_l.language_id = l.id
        WHERE l_l.level_id = ?",
        level_id,
    )
    .fetch_all(db)
    .await?;

    Ok(convert_languages(languages))
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<Language> {
    let language = sqlx::query_as!(RawLanguage, "SELECT * FROM languages WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    Ok(convert_language(language))
}

pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<Language>> {
    let languages = sqlx::query_as!(RawLanguage, "SELECT * FROM languages",)
        .fetch_all(db)
        .await?;

    Ok(convert_languages(languages))
}

pub async fn get_all_non_secret(db: &Pool<Sqlite>) -> WWResult<Vec<Language>> {
    let languages = sqlx::query_as!(RawLanguage, "SELECT * FROM languages WHERE secret = false",)
        .fetch_all(db)
        .await?;

    Ok(convert_languages(languages))
}

pub async fn get_languages_by_ids(db: &Pool<Sqlite>, ids: Vec<i64>) -> WWResult<Vec<Language>> {
    let languages = futures::stream::iter(ids)
        .map(|id| async move { get(db, id).await })
        .buffered(100)
        .try_collect()
        .await?;

    Ok(languages)
}
