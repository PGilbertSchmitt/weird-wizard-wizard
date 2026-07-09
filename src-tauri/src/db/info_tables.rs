use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{import::TableRow, WWResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoTable {
    pub id: i64,
    pub name: String,
    pub kind: TableKind,
    pub key_label: String,
    pub value_label: String,
}

impl InfoTable {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        info_tables: &Vec<TableRow>,
    ) -> WWResult<()> {
        let mut seen = HashMap::<String, i64>::new();

        for row in info_tables {
            if !seen.contains_key(&row.table_id) {
                let table_id = row.table_id.clone();
                // First line for a given table_id holds header labels and table type
                println!("Giving {} kind {:?}", row.table_id, row.table_type);
                let record = sqlx::query!(
                    "INSERT INTO info_tables (name, kind, key_label, value_label) VALUES (?, ?, ?, ?)",
                    row.table_id,
                    row.table_type,
                    row.key,
                    row.value,
                ).execute(&mut *tx).await?;
                seen.insert(table_id, record.last_insert_rowid());
            } else {
                // Remaining lines for a given table_id hold the table entries
                let id = *seen.get(&row.table_id).unwrap();
                sqlx::query!(
                    "INSERT INTO info_table_rows (info_table_id, key, value) VALUES (?, ?, ?)",
                    id,
                    row.key,
                    row.value,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        Ok(())
    }
}

#[derive(TS, Debug, Serialize, Deserialize, sqlx::Type)]
#[ts(export, export_to = "info_tables.ts")]
#[sqlx(type_name = "TEXT")]
pub enum TableKind {
    TABLE,
    BLOCK,
    ROLL,
}

impl From<String> for TableKind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "TABLE" => Self::TABLE,
            "BLOCK" => Self::BLOCK,
            "ROLL" => Self::ROLL,
            _ => Self::TABLE, // TABLE is the most generic, so works as a fallback
        }
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "info_tables.ts")]
pub struct InfoTableRow {
    key: String,
    value: String,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "info_tables.ts")]
pub struct FullInfoTable {
    pub id: i64,
    pub name: String,
    pub kind: TableKind,
    pub key_label: String,
    pub value_label: String,
    pub rows: Vec<InfoTableRow>,
}

impl FullInfoTable {
    pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<Self> {
        let info_table = sqlx::query_as!(InfoTable, "SELECT * FROM info_tables WHERE id = ?", id,)
            .fetch_one(db)
            .await?;

        let info_table_rows = sqlx::query_as!(
            InfoTableRow,
            "SELECT key, value FROM info_table_rows WHERE info_table_id = ?",
            id,
        )
        .fetch_all(db)
        .await?;

        Ok(Self {
            id: info_table.id,
            name: info_table.name,
            kind: info_table.kind,
            key_label: info_table.key_label,
            value_label: info_table.value_label,
            rows: info_table_rows,
        })
    }
}
