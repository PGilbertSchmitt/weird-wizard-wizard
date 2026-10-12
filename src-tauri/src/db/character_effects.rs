use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use sqlx::{Pool, Sqlite};

use crate::{
    db::etc::ChoiceDuration, modifiers::FullModifier, util::db_boolean, WWError::Generic, WWResult,
};

#[derive(Debug)]
struct RawCharacterEffect {
    id: i64,
    choice_key: String,
    selection: String,
    dismissable: Option<String>,
    duration: ChoiceDuration,
    cast_hour: i64,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct CharacterEffect {
    pub id: i64,
    pub selection: String,
    pub dismissable: bool,
    pub duration: ChoiceDuration,
    pub cast_hour: i64,
}

pub async fn get_for_character(
    db: &Pool<Sqlite>,
    id: i64,
) -> WWResult<HashMap<String, CharacterEffect>> {
    let choices = sqlx::query_as!(
        RawCharacterEffect,
        "SELECT id, choice_key, selection, dismissable, duration, cast_hour
        FROM character_effects WHERE character_id = ?",
        id
    )
    .fetch_all(db)
    .await?;

    let mut choice_map = HashMap::new();
    for choice in choices {
        choice_map.insert(
            choice.choice_key,
            CharacterEffect {
                id: choice.id,
                selection: choice.selection,
                dismissable: db_boolean(choice.dismissable),
                duration: choice.duration,
                cast_hour: choice.cast_hour,
            },
        );
    }

    Ok(choice_map)
}

pub async fn save_choices(
    db: &Pool<Sqlite>,
    character_id: i64,
    modifier: FullModifier,
    values: Vec<String>,
) -> WWResult<()> {
    let choice_keys = modifier.keys();
    if choice_keys.len() != values.len() {
        return Err(Generic(format!("Choice could not be saved, number of saved values does not match the number of choice keys")));
    }
    let (dismissable, duration) = modifier.mod_details.when.to_choice_data();
    let cast_hour = sqlx::query_scalar!(
        "SELECT current_hour FROM characters WHERE id = ?",
        character_id
    )
    .fetch_one(db)
    .await?;

    for (key, value) in choice_keys.into_iter().zip(values.into_iter()) {
        sqlx::query!(
            "INSERT INTO character_effects (
                character_id,
                choice_key,
                selection,
                dismissable,
                duration,
                cast_hour
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT(character_id, choice_key) DO UPDATE SET selection = $3",
            character_id,
            key,
            value,
            dismissable,
            duration,
            cast_hour
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

pub async fn delete_choice(
    db: &Pool<Sqlite>,
    character_id: i64,
    modifier: FullModifier,
) -> WWResult<()> {
    let choice_keys = modifier.keys();

    for key in choice_keys.into_iter() {
        sqlx::query!(
            "DELETE FROM character_effects WHERE character_id = ? AND choice_key = ?",
            character_id,
            key,
        )
        .execute(db)
        .await?;
    }

    Ok(())
}
