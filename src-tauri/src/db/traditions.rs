use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::info_tables::FullInfoTable,
    import::{NameToId, TraditionRow},
    WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tradition {
    id: i64,
    name: String,
    blurb: String,
    description: String,
    special_info: Option<String>,
    info_table_id: Option<i64>,
}

impl Tradition {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        traditions: &Vec<TraditionRow>,
        table_map: &NameToId,
    ) -> WWResult<NameToId> {
        let mut trad_map = NameToId::new("tradition");

        for row in traditions {
            let name = row.name.clone();
            let table_id = table_map.get_id_from_opt(&row.table)?;
            let record = sqlx::query!(
                "INSERT INTO traditions (name, blurb, description, special_info, info_table_id) VALUES (?, ?, ?, ?, ?)",
                row.name,
                row.blurb,
                row.description,
                row.special_info,
                table_id,
            )
            .execute(&mut *tx)
            .await?;

            trad_map.insert(name, record.last_insert_rowid());
        }

        Ok(trad_map)
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "magic.ts")]
pub struct FullTradition {
    id: i64,
    name: String,
    blurb: String,
    description: String,
    special_info: Option<String>,
    into_table: Option<FullInfoTable>,
}

impl FullTradition {
    pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullTradition> {
        let tradition = sqlx::query_as!(Tradition, "SELECT * FROM traditions WHERE id = ?", id)
            .fetch_one(db)
            .await?;

        let table = FullInfoTable::get_from_opt(db, tradition.info_table_id).await?;

        Ok(FullTradition {
            id,
            name: tradition.name,
            blurb: tradition.blurb,
            description: tradition.description,
            special_info: tradition.special_info,
            into_table: table,
        })
    }
}
