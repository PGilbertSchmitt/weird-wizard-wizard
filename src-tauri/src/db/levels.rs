use serde::{Deserialize, Serialize};
use sqlx::SqliteConnection;

use crate::{
    db::etc,
    import::{pipe_separate, NamePairToId, NameToId, PathLevelRow},
    WWError, WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    id: i64,
    path: i64,
    level: i64,
    add_health: i64,
    add_nat_def: i64,
    add_arm_def: i64,
    add_bonus_dmg: i64,
    add_speed: i64,
    trad_choices: i64,
    lang_choices: i64,
    novice_spells: i64,
    expert_spells: i64,
    master_spells: i64,
    size: Option<etc::Size>,
}

impl Level {
    pub async fn insert_all(
        tx: &mut SqliteConnection,
        levels: &Vec<PathLevelRow>,
        path_map: &NameToId,
        trad_map: &NameToId,
        language_map: &NameToId,
        speed_trait_map: &NameToId,
        path_talent_map: &NamePairToId,
    ) -> WWResult<()> {
        for row in levels {
            let path_id = path_map.get_id(&row.path)?;
            let record = sqlx::query!(
                "INSERT INTO levels (
                    path,
                    level,
                    add_health,
                    add_nat_def,
                    add_arm_def,
                    add_bonus_dmg,
                    add_speed,
                    trad_choices,
                    lang_choices,
                    novice_spells,
                    expert_spells,
                    master_spells,
                    size
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                path_id,
                row.level,
                row.health,
                row.nat_def,
                row.armed_def,
                row.bonus_dmg,
                row.speed,
                row.trad_choices,
                row.lang_choices,
                row.novice_spells,
                row.expert_spells,
                row.master_spells,
                row.size
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                WWError::Generic(format!(
                    "Encountered error while seeding {} level {}: {}",
                    path_id, row.level, e
                ))
            })?;
            let level_id = record.last_insert_rowid();

            for tradition in pipe_separate(&row.traditions) {
                let trad_id = trad_map.get_id(&tradition)?;
                sqlx::query!(
                    "INSERT INTO level_traditions (level_id, tradition_id) VALUES (?, ?)",
                    level_id,
                    trad_id,
                )
                .execute(&mut *tx)
                .await?;
            }

            for language in pipe_separate(&row.languages) {
                let lang_id = language_map.get_id(&language)?;
                sqlx::query!(
                    "INSERT INTO level_languages (level_id, language_id) VALUES (?, ?)",
                    level_id,
                    lang_id,
                )
                .execute(&mut *tx)
                .await?;
            }

            for speed_trait in pipe_separate(&row.speed_traits) {
                let speed_trait_id = speed_trait_map.get_id(&speed_trait)?;
                sqlx::query!(
                    "INSERT INTO level_speed_traits (level_id, speed_trait_id) VALUES (?, ?)",
                    level_id,
                    speed_trait_id,
                )
                .execute(&mut *tx)
                .await?;
            }

            for talent in pipe_separate(&row.talents) {
                let path_talent_id = path_talent_map.get_id(&(talent, row.path.clone()))?;
                sqlx::query!(
                    "INSERT INTO level_talents (level_id, path_talent_id) VALUES (?, ?)",
                    level_id,
                    path_talent_id,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        Ok(())
    }
}
