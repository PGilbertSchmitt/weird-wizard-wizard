use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    mem,
    sync::Arc,
};
use ts_rs::TS;

use dashmap::DashSet;
use sqlx::{Pool, Sqlite};

use crate::{
    db::{
        choice_selections,
        etc::ChoiceDuration,
        magic_talents::{self, FullMagicTalent},
        path_talents::{self, FullPathTalent},
        spells::{self, FullSpell},
    },
    mod_dsl::ast::{ChooseTarget, GrantTarget, LoseTarget, Modifier, OverrideTarget, Target},
    modifiers::{FullModifier, HasModifiers},
    util::db_boolean,
    WWError::Generic,
    WWResult,
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
