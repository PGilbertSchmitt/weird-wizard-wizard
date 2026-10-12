use futures::{StreamExt, TryStreamExt};
use sqlx::{Pool, Sqlite};

use crate::{mod_dsl::ast::WhenDuration, WWResult};

async fn _pass_time(db: &Pool<Sqlite>, character_id: i64, duration_str: String) -> WWResult<()> {
    sqlx::query!(
        "DELETE FROM character_effects WHERE character_id = ? AND duration = ?",
        character_id,
        duration_str,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn pass_time(
    db: &Pool<Sqlite>,
    character_id: i64,
    duration: WhenDuration,
) -> WWResult<()> {
    let duration_strings = duration.contained_duration_strings();
    let len = duration_strings.len();
    futures::stream::iter(duration_strings.clone())
        .map(|duration_str| async move { _pass_time(db, character_id, duration_str).await })
        .buffered(len)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(())
}
