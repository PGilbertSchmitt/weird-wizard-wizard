use sqlx::SqliteConnection;

use crate::mod_dsl::ast::{ChooseTarget, GrantTarget, LoseTarget, Modifier, Target};

pub async fn validate_modifier(
    tx: &mut SqliteConnection,
    modifier: Modifier,
) -> Result<(), String> {
    match modifier.target {
        Target::Grant(grant_target) => validate_grant_target(tx, grant_target).await,
        Target::Lose(lose_target) => validate_lose_target(tx, lose_target).await,
        Target::Choose(choose_target, _) => validate_choose_target(tx, choose_target).await,
        _ => Ok(()),
    }
}

async fn validate_grant_target(
    tx: &mut SqliteConnection,
    grant_target: GrantTarget,
) -> Result<(), String> {
    let target_str = format!("Error constructing grant target '{grant_target:?}'");
    match grant_target {
        GrantTarget::Language(items) => {
            for item in items {
                sqlx::query!("SELECT id FROM languages WHERE name = ?", item)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|_| format!("{target_str:?}, could not find language '{item}'"))?;
            }
            Ok(())
        }
        GrantTarget::Tradition(items) => {
            for item in items {
                find_tradition(&mut *tx, item, &target_str).await?;
            }
            Ok(())
        }
        GrantTarget::Sense(items) => {
            for (item, _) in items {
                sqlx::query!("SELECT id FROM senses WHERE name = ?", item)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|_| format!("{target_str:?}, could not find sense '{item}'"))?;
            }
            Ok(())
        }
        GrantTarget::SpeedTrait(items) => {
            for (item, _) in items {
                sqlx::query!("SELECT id FROM speed_traits WHERE name = ?", item)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|_| format!("{target_str:?}, could not find speed trait '{item}'"))?;
            }
            Ok(())
        }
        GrantTarget::Talent(source, name) => {
            find_path_talent_by_source_and_name(&mut *tx, source, name, &target_str).await
        }
        GrantTarget::MagicTalent(tradition, name) => {
            find_magic_talent_by_tradition_and_name(&mut *tx, tradition, name, &target_str).await
        }
        _ => Ok(()),
    }
}

async fn validate_lose_target(
    tx: &mut SqliteConnection,
    lose_target: LoseTarget,
) -> Result<(), String> {
    let target_str = format!("Error constructing lose target '{lose_target:?}'");
    match lose_target {
        LoseTarget::Talent(source, name) => {
            find_path_talent_by_source_and_name(&mut *tx, source, name, &target_str).await
        }
        LoseTarget::MagicTalent(tradition, name) => {
            find_magic_talent_by_tradition_and_name(&mut *tx, tradition, name, &target_str).await
        }
    }
}

async fn validate_choose_target(
    tx: &mut SqliteConnection,
    choose_target: ChooseTarget,
) -> Result<(), String> {
    let target_str = format!("Error constructing choose target '{choose_target:?}'");
    match choose_target {
        ChooseTarget::NoviceSpellFrom(_, items)
        | ChooseTarget::ExpertSpellFrom(_, items)
        | ChooseTarget::MasterSpellFrom(_, items) => {
            for item in items {
                find_tradition(&mut *tx, item, &target_str).await?;
            }
            Ok(())
        }
        ChooseTarget::Select(_, name) => {
            sqlx::query!("SELECT id FROM choice_tables WHERE name = ?", name)
                .fetch_one(&mut *tx)
                .await
                .map_err(|_| format!("{target_str:?}, could not find choice selection '{name}'"))?;
            Ok(())
        }
        _ => Ok(()),
    }
}

// It was easier to recreate these simple queries

async fn find_tradition(
    tx: &mut SqliteConnection,
    name: String,
    target_str: &str,
) -> Result<(), String> {
    if name == "ANY" {
        return Ok(());
    }

    sqlx::query!("SELECT id FROM traditions WHERE name = ?", name)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| format!("{target_str:?}, could not find tradition '{name}'"))?;
    Ok(())
}

async fn find_path_talent_by_source_and_name(
    tx: &mut SqliteConnection,
    source: String,
    name: String,
    target_str: &str,
) -> Result<(), String> {
    sqlx::query!(
        "SELECT id FROM path_talents WHERE source = ? AND name = ?",
        source,
        name,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| {
        format!("{target_str:?}, could not find path talent '{name}' with source '{source}'")
    })?;
    Ok(())
}

async fn find_magic_talent_by_tradition_and_name(
    tx: &mut SqliteConnection,
    tradition: String,
    name: String,
    target_str: &str,
) -> Result<(), String> {
    sqlx::query!(
        "SELECT mt.*, t.name as tradition_name FROM magic_talents mt
        JOIN traditions t ON t.id = mt.tradition_id
        WHERE t.name = ? AND mt.name = ?",
        tradition,
        name,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| {
        format!("{target_str:?}, could not find magic talent '{name}' with tradition '{tradition}'")
    })?;
    Ok(())
}
