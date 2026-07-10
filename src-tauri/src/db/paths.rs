use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::etc::PathKind,
    import::{is_affirmative, ExpertOrMasterPathRow, NameToId, NovicePathRow},
    WWError, WWResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    name: String,
    path_kind: PathKind,
    category: String,
    description: String,
    rec_str: Option<i64>,
    rec_agl: Option<i64>,
    rec_int: Option<i64>,
    rec_will: Option<i64>,
    ancestry: Option<i64>,
}

impl Path {
    async fn insert_one(tx: &mut SqliteConnection, path: Path) -> WWResult<i64> {
        let record = sqlx::query!(
            "INSERT INTO paths (
                name,
                path_kind,
                category,
                description,
                rec_str,
                rec_agl,
                rec_int,
                rec_will,
                ancestry
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            path.name,
            path.path_kind,
            path.category,
            path.description,
            path.rec_str,
            path.rec_agl,
            path.rec_int,
            path.rec_will,
            path.ancestry,
        )
        .execute(&mut *tx)
        .await?;
        Ok(record.last_insert_rowid())
    }

    pub async fn insert_all_novice(
        tx: &mut SqliteConnection,
        paths: &Vec<NovicePathRow>,
        ancestry_map: &NameToId,
    ) -> WWResult<NameToId> {
        let mut path_map = NameToId::new("novice_path");
        for row in paths {
            let (rec_str, rec_agl, rec_int, rec_will) =
                Self::split_lvl_1_scores(&row.init_scores_lvl_1)?;
            let is_ancestry = is_affirmative(row.origin_locked.as_deref());
            let category = if is_ancestry {
                "Ancestry Path"
            } else {
                "Novice Path"
            };
            let ancestry = if is_ancestry {
                Some(ancestry_map.get_id(&row.name).map_err(|_| {
                    WWError::Generic(format!("Could not find ancestry with name '{}'", &row.name))
                })?)
            } else {
                None
            };
            let id = Self::insert_one(
                tx,
                Path {
                    name: row.name.clone(),
                    path_kind: PathKind::Novice,
                    category: category.into(),
                    description: row.description.clone(),
                    rec_str: Some(rec_str),
                    rec_agl: Some(rec_agl),
                    rec_int: Some(rec_int),
                    rec_will: Some(rec_will),
                    ancestry,
                },
            )
            .await?;

            path_map.insert(row.name.clone(), id);
        }

        Ok(path_map)
    }

    fn split_lvl_1_scores(scores: &str) -> WWResult<(i64, i64, i64, i64)> {
        let split: Vec<&str> = scores.split("|").collect();
        if split.len() != 4 {
            return Err(WWError::Generic(format!("")));
        }

        let as_nums = split
            .iter()
            .map(|s| {
                s.parse::<i64>().map_err(|_| {
                    WWError::Generic(format!("Failed to parse novice init scores '{scores}'"))
                })
            })
            .collect::<WWResult<Vec<i64>>>()?;
        Ok((
            *as_nums.get(0).unwrap(),
            *as_nums.get(1).unwrap(),
            *as_nums.get(2).unwrap(),
            *as_nums.get(3).unwrap(),
        ))
    }

    pub async fn insert_all_expert_or_master(
        tx: &mut SqliteConnection,
        paths: &Vec<ExpertOrMasterPathRow>,
        path_kind: PathKind,
    ) -> WWResult<NameToId> {
        let mut path_map = NameToId::new(if path_kind == PathKind::Expert {
            "expert path"
        } else {
            "master path"
        });
        for row in paths {
            let id = Self::insert_one(
                tx,
                Path {
                    name: row.name.clone(),
                    path_kind,
                    category: row.sub_path.clone(),
                    description: row.description.clone(),
                    rec_str: None,
                    rec_agl: None,
                    rec_int: None,
                    rec_will: None,
                    ancestry: None,
                },
            )
            .await?;

            path_map.insert(row.name.clone(), id);
        }

        Ok(path_map)
    }
}

// pub struct FullPath {

// }
