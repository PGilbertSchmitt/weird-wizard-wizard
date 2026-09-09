use std::collections::{hash_map::Entry, HashMap};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqliteConnection};
use ts_rs::TS;

use crate::{
    db::professions::{self, Profession},
    import::ProfessionCategoryRow,
    WWError, WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct ProfessionCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}

pub async fn insert_all(
    tx: &mut SqliteConnection,
    rows: &Vec<ProfessionCategoryRow>,
) -> WWResult<()> {
    for row in rows {
        sqlx::query!(
            "INSERT INTO profession_categories (name, description) VALUES (?, ?)",
            row.name,
            row.description
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            WWError::Generic(format!(
                "Encountered error while seeding profession category {}: {}",
                row.name, e
            ))
        })?;
    }
    Ok(())
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "other_info.ts")]
pub struct FullProfessionCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub professions: Vec<Profession>,
}

pub async fn get_all(db: &Pool<Sqlite>) -> WWResult<Vec<FullProfessionCategory>> {
    let (raw_categories, raw_professions) = futures::join!(
        sqlx::query_as!(ProfessionCategory, "SELECT * FROM profession_categories").fetch_all(db),
        professions::get_all(db)
    );

    let mut profession_map = HashMap::<String, Vec<Profession>>::new();
    for prof in raw_professions? {
        match profession_map.entry(prof.category.clone()) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(prof);
            }
            Entry::Vacant(h) => {
                h.insert_entry(vec![prof]);
            }
        }
    }
    let categories = raw_categories?
        .into_iter()
        .map(|cat| {
            let professions = profession_map.remove(&cat.name).unwrap_or(Vec::new());
            FullProfessionCategory {
                id: cat.id,
                name: cat.name,
                description: cat.description,
                professions,
            }
        })
        .collect();

    Ok(categories)
}
