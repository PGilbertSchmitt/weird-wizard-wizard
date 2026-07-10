use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::{etc::TalentRestore, info_tables::FullInfoTable, option_blocks::FullOptionBlock},
    import::{MagicTalentRow, NameToId},
    WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MagicTalent {
    id: i64,
    tradition_id: i64,
    name: String,
    description: String,
    charges: Option<String>,
    restore: TalentRestore,
    activate: String,
    info_table_id: Option<i64>,
    option_block_id: Option<i64>,
}

impl MagicTalent {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        magic_talents: &Vec<MagicTalentRow>,
        trad_map: &NameToId,
        table_map: &NameToId,
        option_map: &NameToId,
    ) -> WWResult<()> {
        for row in magic_talents {
            let tradition_id = trad_map.get_id(&row.tradition)?;
            let table_id = table_map.get_id_from_opt(&row.table)?;
            let options_id = option_map.get_id_from_opt(&row.options)?;

            sqlx::query!(
                "INSERT INTO magic_talents (
                    tradition_id,
                    name,
                    description,
                    charges,
                    restore,
                    activate,
                    info_table_id,
                    option_block_id
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                tradition_id,
                row.talent_name,
                row.description,
                row.charges,
                row.restore,
                row.activate,
                table_id,
                options_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(())
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "magic.ts")]
pub struct FullMagicTalent {
    id: i64,
    tradition_id: i64,
    tradition_name: String,
    name: String,
    description: String,
    charges: Option<String>,
    restore: TalentRestore,
    activate: String,
    info_table_id: Option<FullInfoTable>,
    option_block_id: Option<FullOptionBlock>,
}

impl FullMagicTalent {
    pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullMagicTalent> {
        let talent = sqlx::query_as!(
            MagicTalentWithTradName,
            "SELECT mt.*, t.name as tradition_name
            FROM magic_talents mt
            JOIN traditions t ON t.id = mt.tradition_id
            WHERE mt.id = ?",
            id
        )
        .fetch_one(db)
        .await?;

        let info_table = FullInfoTable::get_from_opt(db, talent.info_table_id).await?;
        let option_block = FullOptionBlock::get_from_opt(db, talent.option_block_id).await?;

        Ok(FullMagicTalent {
            id,
            tradition_id: talent.tradition_id,
            tradition_name: talent.tradition_name,
            name: talent.name,
            description: talent.description,
            charges: talent.charges,
            restore: talent.restore,
            activate: talent.activate,
            info_table_id: info_table,
            option_block_id: option_block,
        })
    }
}

// An intermediary struct for one fewer query
#[derive(Serialize, Deserialize)]
struct MagicTalentWithTradName {
    id: i64,
    tradition_id: i64,
    tradition_name: String,
    name: String,
    description: String,
    charges: Option<String>,
    restore: TalentRestore,
    activate: String,
    info_table_id: Option<i64>,
    option_block_id: Option<i64>,
}
