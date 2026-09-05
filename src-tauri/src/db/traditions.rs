use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    WWResult, db::{etc::PathKind, info_tables::{self, FullInfoTable}, magic_talents::{self, FullMagicTalent}, spells::{self, FullSpell}}, import::{NameToId, TraditionRow},
};

#[derive(Debug, Serialize, Deserialize)]
struct RawTradition {
    id: i64,
    name: String,
    blurb: String,
    description: String,
    special_info: Option<String>,
    info_table_id: Option<i64>,
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
    talents: Vec<FullMagicTalent>,
    novice_spells: Vec<FullSpell>,
    expert_spells: Vec<FullSpell>,
    master_spells: Vec<FullSpell>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "magic.ts")]
pub struct TraditionIndexItem {
    id: i64,
    name: String,
    blurb: String,
}

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

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullTradition> {
    let tradition = sqlx::query_as!(RawTradition, "SELECT * FROM traditions WHERE id = ?", id)
        .fetch_one(db)
        .await?;

    let (table, talents, spells) = futures::join!(
        info_tables::get_from_opt(db, tradition.info_table_id),
        magic_talents::get_for_tradition(db, id),
        spells::get_for_tradition(db, id),
    );

    let mut novice_spells = Vec::new();
    let mut expert_spells = Vec::new();
    let mut master_spells = Vec::new();

    for spell in spells? {
        match spell.path_kind {
            PathKind::Novice => novice_spells.push(spell),
            PathKind::Expert => expert_spells.push(spell),
            PathKind::Master => master_spells.push(spell),
        }
    }

    Ok(FullTradition {
        id,
        name: tradition.name,
        blurb: tradition.blurb,
        description: tradition.description,
        special_info: tradition.special_info,
        into_table: table?,
        talents: talents?,
        novice_spells,
        expert_spells,
        master_spells,
    })
}

pub async fn get_index(db: &Pool<Sqlite>) -> WWResult<Vec<TraditionIndexItem>> {
    Ok(sqlx::query_as!(
        TraditionIndexItem,
        "SELECT id, name, blurb FROM traditions"
    )
    .fetch_all(db)
    .await?)
}
