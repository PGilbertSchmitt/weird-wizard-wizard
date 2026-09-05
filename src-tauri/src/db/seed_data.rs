use sqlx::{Pool, Sqlite};

use crate::WWResult;

pub async fn is_seeded(db: &Pool<Sqlite>) -> WWResult<bool> {
    let (ancestry_count, novice_path_count) = futures::join!(
        sqlx::query_scalar!("SELECT COUNT(*) FROM ancestries").fetch_one(db),
        sqlx::query_scalar!("SELECT COUNT(*) FROM paths WHERE path_kind = 'Novice'").fetch_one(db),
    );

    Ok(ancestry_count? > 0 && novice_path_count? > 0)
}