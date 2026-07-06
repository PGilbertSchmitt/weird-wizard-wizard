use crate::{WWError, WWResult};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use ts_rs::TS;

mod ancestries;

pub struct Database {
    pub pool: Pool<Sqlite>,
    pub path: PathBuf,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> WWResult<Self> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app directory");

        fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("www.db");

        println!("---------------------------------");
        println!("|> DB Path: {:#?}", db_path);
        println!("---------------------------------");

        let db = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(db_path.clone())
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(db).await?;

        sqlx::migrate!("db/migrations").run(&pool).await?;

        Ok(Self {
            pool,
            path: db_path,
        })
    }
}

pub struct DatabaseState {
    pub pool: Pool<Sqlite>,
    pub path: PathBuf,
}

pub trait StoreModel: Sized + DeserializeOwned + Serialize {
    const TABLE_NAME: &'static str;
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "etc.ts")]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl TryFrom<&str> for Size {
    type Error = WWError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "sm" => Ok(Self::Sm),
            "md" => Ok(Self::Md),
            "lg" => Ok(Self::Lg),
            sz => Err(WWError::Generic(format!("{} is not a valid size", sz))),
        }
    }
}
