use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::{
        etc::PathKind,
        levels::{self, FullLevel},
    },
    import::{is_affirmative, ExpertOrMasterPathRow, NameToId, NovicePathRow},
    WWError, WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize)]
pub struct CreatePath {
    name: String,
    path_kind: PathKind,
    category: String,
    description: String,
    rec_str: Option<i64>,
    rec_agl: Option<i64>,
    rec_int: Option<i64>,
    rec_will: Option<i64>,
    ancestry_id: Option<i64>,
}

async fn insert_one(tx: &mut SqliteConnection, path: CreatePath) -> WWResult<i64> {
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
            ancestry_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        path.name,
        path.path_kind,
        path.category,
        path.description,
        path.rec_str,
        path.rec_agl,
        path.rec_int,
        path.rec_will,
        path.ancestry_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        WWError::Generic(format!(
            "Encountered error while seeding {} path {}: {}",
            path.path_kind, path.name, e
        ))
    })?;
    Ok(record.last_insert_rowid())
}

pub async fn insert_all_novice(
    tx: &mut SqliteConnection,
    paths: &Vec<NovicePathRow>,
    ancestry_map: &NameToId,
) -> WWResult<NameToId> {
    let mut path_map = NameToId::new("novice_path");
    for row in paths {
        let (rec_str, rec_agl, rec_int, rec_will) = split_lvl_1_scores(&row.init_scores_lvl_1)?;
        let is_ancestry = is_affirmative(row.origin_locked.as_deref());
        let category = if is_ancestry {
            "Ancestry Path"
        } else {
            "Novice Path"
        };
        let ancestry_id = if is_ancestry {
            Some(ancestry_map.get_id(&row.name).map_err(|_| {
                WWError::Generic(format!("Could not find ancestry with name '{}'", &row.name))
            })?)
        } else {
            None
        };
        let id = insert_one(
            tx,
            CreatePath {
                name: row.name.clone(),
                path_kind: PathKind::Novice,
                category: category.into(),
                description: row.description.clone(),
                rec_str: Some(rec_str),
                rec_agl: Some(rec_agl),
                rec_int: Some(rec_int),
                rec_will: Some(rec_will),
                ancestry_id,
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
        let id = insert_one(
            tx,
            CreatePath {
                name: row.name.clone(),
                path_kind,
                category: row.sub_path.clone(),
                description: row.description.clone(),
                rec_str: None,
                rec_agl: None,
                rec_int: None,
                rec_will: None,
                ancestry_id: None,
            },
        )
        .await?;

        path_map.insert(row.name.clone(), id);
    }

    Ok(path_map)
}

pub struct RawPath {
    id: i64,
    name: String,
    path_kind: PathKind,
    category: String,
    description: String,
    rec_str: Option<i64>,
    rec_agl: Option<i64>,
    rec_int: Option<i64>,
    rec_will: Option<i64>,
    ancestry_id: Option<i64>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct PathAncestry {
    id: i64,
    name: String,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct NovicePath {
    id: i64,
    name: String,
    path_kind: PathKind,
    category: String,
    description: String,
    levels: Vec<FullLevel>,
    rec_str: i64,
    rec_agl: i64,
    rec_int: i64,
    rec_will: i64,
    ancestry: Option<PathAncestry>,
}

// This is the "full" path in the sense that we only need these values
// for typical play. The other fields are only needed for Novice Paths,
// and only during character creation.
#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct FullPath {
    id: i64,
    name: String,
    path_kind: PathKind,
    category: String,
    description: String,
    levels: Vec<FullLevel>,
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullPath> {
    let (path, levels) = futures::join!(
        sqlx::query_as!(RawPath, "SELECT * FROM paths WHERE id = ?", id).fetch_one(db),
        levels::get_for_path(db, id),
    );

    let path = path?;
    let levels = levels?;

    Ok(FullPath {
        id: path.id,
        name: path.name,
        path_kind: path.path_kind,
        category: path.category,
        description: path.description,
        levels,
    })
}

// This is only used for character creation when picking novice paths specifically
pub async fn get_novice_path(db: &Pool<Sqlite>, id: i64) -> WWResult<NovicePath> {
    let (path, levels) = futures::join!(
        sqlx::query_as!(RawPath, "SELECT * FROM paths WHERE id = ?", id).fetch_one(db),
        levels::get_for_path(db, id),
    );

    let path = path?;
    let levels = levels?;

    let ancestry = match path.ancestry_id {
        Some(ancestry_id) => Some(
            sqlx::query_as!(
                PathAncestry,
                "SELECT id, name FROM ancestries WHERE id = ?",
                ancestry_id
            )
            .fetch_one(db)
            .await?,
        ),
        None => None,
    };

    Ok(NovicePath {
        id: path.id,
        name: path.name,
        path_kind: path.path_kind,
        category: path.category,
        description: path.description,
        levels,
        rec_str: path.rec_str.unwrap_or(10),
        rec_agl: path.rec_agl.unwrap_or(10),
        rec_int: path.rec_int.unwrap_or(10),
        rec_will: path.rec_will.unwrap_or(10),
        ancestry,
    })
}
