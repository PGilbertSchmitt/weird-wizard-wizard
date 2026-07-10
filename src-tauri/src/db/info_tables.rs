use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    import::{NameToId, TableRow},
    WWResult,
};

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
    ) -> WWResult<NameToId> {
        let mut table_map = NameToId::new("info_table");

        for row in info_tables {
            if !table_map.has(&row.table_id) {
                let table_id = row.table_id.clone();
                // First line for a given table_id holds header labels and table type
                let record = sqlx::query!(
                    "INSERT INTO info_tables (name, kind, key_label, value_label) VALUES (?, ?, ?, ?)",
                    row.table_id,
                    row.table_type,
                    row.key,
                    row.value,
                ).execute(&mut *tx).await?;
                table_map.insert(table_id, record.last_insert_rowid());
            } else {
                // Remaining lines for a given table_id hold the table entries
                let id = table_map.get_id(&row.table_id)?;
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

        Ok(table_map)
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
pub struct Entry {
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
    pub entries: Vec<Entry>,
}

impl FullInfoTable {
    pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<Self> {
        let info_table = sqlx::query_as!(InfoTable, "SELECT * FROM info_tables WHERE id = ?", id,)
            .fetch_one(db)
            .await?;

        let info_table_rows = sqlx::query_as!(
            Entry,
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
            entries: info_table_rows,
        })
    }

    pub async fn get_from_opt(db: &Pool<Sqlite>, id: Option<i64>) -> WWResult<Option<Self>> {
        Ok(if let Some(id_unwrapped) = id {
            Some(Self::get(db, id_unwrapped).await?)
        } else {
            None
        })
    }
}
