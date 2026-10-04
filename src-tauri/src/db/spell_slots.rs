use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use ts_rs::TS;

use crate::{WWResult, util::clamp};

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "magic.ts")]
pub struct SpellSlotInfo {
    perm_used: i64,
    perm_total: i64,
    temp_used: i64,
    temp_total: i64,
}

// impl SpellSlotInfo {
//     fn increment_perm(&mut self) {
//         self.
//     }
// }

pub async fn get(
    db: &Pool<Sqlite>,
    character_id: i64,
    spell_id: i64,
) -> WWResult<Option<SpellSlotInfo>> {
    let slot_info = sqlx::query_as!(
        SpellSlotInfo,
        "SELECT perm_used, temp_used, perm_total, temp_total FROM spell_slots WHERE character_id = ? AND spell_id = ?",
        character_id,
        spell_id,
    ).fetch_all(db).await?;

    // There's either 1 or 0
    Ok(slot_info.into_iter().next())
}

pub async fn cast(
    db: &Pool<Sqlite>,
    character_id: i64,
    spell_id: i64,
    perm_count: i64,
    temp_count: i64,
) -> WWResult<()> {
    let cur_info = get(db, character_id, spell_id)
        .await?
        .unwrap_or_else(|| SpellSlotInfo {
            perm_used: 0,
            temp_used: 0,
            perm_total: perm_count,
            temp_total: temp_count,
        });

    // Positive means slot were gained. Negative means slots were lost, and in that case, we need
    // to subtract the delta (by adding it, cuz it's negative) to the saved perm used.
    let temp_delta = temp_count - cur_info.temp_total;
    let cur_temp_used = if temp_delta < 0 {
        clamp(cur_info.temp_used + temp_delta, 0, temp_count)
    } else {
        cur_info.temp_used
    };

    // Same logic for the permanent slots (they're just separated so that changes in temp slots
    // don't affect permanent slots).
    let perm_delta = perm_count - cur_info.perm_total;
    let cur_perm_used = if perm_delta < 0 {
        clamp(cur_info.perm_used + perm_delta, 0, perm_count)
    } else {
        cur_info.perm_used
    };

    let new_info = if cur_temp_used < temp_count {
        SpellSlotInfo {
            perm_used: cur_perm_used,
            temp_used: cur_temp_used + 1,
            perm_total: perm_count,
            temp_total: temp_count,
        }
    } else {
        SpellSlotInfo {
            perm_used: cur_perm_used + 1,
            temp_used: cur_temp_used,
            perm_total: perm_count,
            temp_total: temp_count,
        }
    };

    sqlx::query!(
        "INSERT INTO spell_slots (
            character_id,
            spell_id,
            perm_used,
            temp_used,
            perm_total,
            temp_total
        ) VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT(character_id, spell_id) DO UPDATE SET
        perm_used = $3, temp_used = $4, perm_total = $5, temp_total = $6",
        character_id,
        spell_id,
        new_info.perm_used,
        new_info.temp_used,
        new_info.perm_total,
        new_info.temp_total
    ).execute(db).await?;

    Ok(())
}
