use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::{etc, info_tables::FullInfoTable, option_blocks::FullOptionBlock},
    import::{MagicSpellRow, NameToId},
    WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Spell {
    id: i64,
    tradition_id: i64,
    name: String,
    description: String,
    path_kind: etc::PathKind,
    castings: i64,
    duration: String,
    target: String,
    condition: Option<String>,
    ritual: bool,
    info_table_id: Option<i64>,
    option_block_id: Option<i64>,
}

impl Spell {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        magic_spells: &Vec<MagicSpellRow>,
        trad_map: &NameToId,
        table_map: &NameToId,
        option_map: &NameToId,
    ) -> WWResult<()> {
        for row in magic_spells {
            let tradition_id = trad_map.get_id(&row.tradition)?;
            let table_id = table_map.get_id_from_opt(&row.table)?;
            let options_id = option_map.get_id_from_opt(&row.options)?;

            sqlx::query!(
                "INSERT INTO spells (
                    tradition_id,
                    name,
                    description,
                    path_kind,
                    castings,
                    duration,
                    target,
                    condition,
                    ritual,
                    info_table_id,
                    option_block_id
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                tradition_id,
                row.name,
                row.description,
                row.path_type,
                row.castings,
                row.duration,
                row.target,
                row.condition,
                row.ritual,
                table_id,
                options_id
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(())
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "magic.ts")]
pub struct FullSpell {
    id: i64,
    tradition_id: i64,
    tradition_name: String,
    name: String,
    description: String,
    path_kind: etc::PathKind,
    castings: i64,
    duration: String,
    target: String,
    condition: Option<String>,
    ritual: bool,
    info_table: Option<FullInfoTable>,
    option_block: Option<FullOptionBlock>,
}

impl FullSpell {
    pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullSpell> {
        let spell = sqlx::query_as!(
            SpellWithTradName,
            "SELECT s.*, t.name as tradition_name FROM spells s JOIN traditions t ON t.id = s.tradition_id WHERE s.id = ?",
            id
        ).fetch_one(db).await?;

        let info_table = FullInfoTable::get_from_opt(db, spell.info_table_id).await?;
        let option_block = FullOptionBlock::get_from_opt(db, spell.option_block_id).await?;

        Ok(FullSpell {
            id,
            tradition_id: spell.tradition_id,
            tradition_name: spell.tradition_name,
            name: spell.name,
            description: spell.description,
            path_kind: spell.path_kind,
            castings: spell.castings,
            duration: spell.duration,
            target: spell.target,
            condition: spell.condition,
            ritual: spell.ritual,
            info_table,
            option_block,
        })
    }
}

// An intermediary struct for one fewer query
#[derive(Serialize, Deserialize)]
struct SpellWithTradName {
    id: i64,
    tradition_id: i64,
    tradition_name: String,
    name: String,
    description: String,
    path_kind: etc::PathKind,
    castings: i64,
    duration: String,
    target: String,
    condition: Option<String>,
    ritual: bool,
    info_table_id: Option<i64>,
    option_block_id: Option<i64>,
}
