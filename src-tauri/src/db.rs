use std::{fs, path::PathBuf};

use anyhow::{Ok, Result};
use sqlx::{Pool, Sqlite, SqlitePool};
use tauri::{AppHandle, Manager};

pub struct Database {
    pub pool: Pool<Sqlite>,
    pub path: PathBuf,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self> {
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

#[allow(dead_code)]
pub struct DatabaseState {
    pub pool: Pool<Sqlite>,
    pub path: PathBuf,
}
