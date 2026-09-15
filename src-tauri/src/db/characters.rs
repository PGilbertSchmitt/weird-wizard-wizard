use dashmap::DashSet;
use futures::{FutureExt, StreamExt, TryStreamExt, stream::FuturesUnordered};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, Pool, Sqlite};
use ts_rs::TS;

use crate::{
    WWResult, db::{
        ancestries::{self, FullAncestry}, character_choices::{self, ModifierSelections, collect_from_modifier_tree}, etc::{ChoiceDuration, Size}, immunities::Immunity, languages::Language, levels::FullLevel, paths::{self, FullPath}, professions::{self, Profession}, senses::FullSense, speed_traits::FullSpeedTrait,
    }, mod_dsl::ast::{ChooseTarget, Condition, GrantTarget, Modifier, Target, WhenMod}, modifiers::FullModifier,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct CreateCharacter {
    name: String,
    profession_id: i64,
    ancestry_id: i64,
    novice_path_id: i64,
    strength: i64,
    agility: i64,
    intellect: i64,
    will: i64,
}

#[derive(Debug)]
struct RawCharacter {
    id: i64,
    name: String,
    level: i64,
    health: i64,
    damage: i64,
    strength: i64,
    agility: i64,
    intellect: i64,
    will: i64,
    profession_id: i64,
    ancestry_id: i64,
    novice_path_id: i64,
    expert_path_id: Option<i64>,
    master_path_id: Option<i64>,
    created_at: NaiveDateTime,
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct FullCharacter {
    id: i64,
    name: String,
    level: i64,
    health: i64,
    damage: i64,
    strength: i64,
    agility: i64,
    intellect: i64,
    will: i64,
    profession: Profession,
    ancestry: FullAncestry,
    novice_path: FullPath,
    expert_path: Option<FullPath>,
    master_path: Option<FullPath>,
    created_at: String,

    // Fields derived from ancestries, paths, and choices
    max_health: i64,
    nat_def: i64,
    arm_def: i64,
    speed: i64,
    bonus_dmg: i64,
    speed_traits: Vec<FullSpeedTrait>,
    senses: Vec<FullSense>,
    immunities: Vec<Immunity>,
    languages: Vec<Language>,
    size: Size,
    // choices: Vec<FullModifier>,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct CharacterIndexItem {
    id: i64,
    name: String,
    level: i64,
    ancestry: String,
    novice_path: String,
    expert_path: Option<String>,
    master_path: Option<String>,
}

pub async fn get_index(db: &Pool<Sqlite>) -> WWResult<Vec<CharacterIndexItem>> {
    let rows = sqlx::query_as!(
        CharacterIndexItem,
        "SELECT
            c.id,
            c.name,
            c.level,
            a.name AS ancestry,
            np.name AS novice_path,
            ep.name AS expert_path,
            mp.name AS master_path
        FROM characters c
        JOIN ancestries a ON a.id = c.ancestry_id
        JOIN paths np ON np.id = c.novice_path_id
        LEFT JOIN paths ep ON ep.id = c.expert_path_id
        LEFT JOIN paths mp ON mp.id = c.master_path_id"
    )
    .fetch_all(db)
    .await?;

    Ok(rows)
}

pub async fn create_character(db: &Pool<Sqlite>, character_info: CreateCharacter) -> WWResult<i64> {
    // Initial character max health is determined by the chosen ancestry and the first level of the chosen Novice path.
    let (ancestry_health, path_health) = futures::join!(
        sqlx::query_scalar!(
            "SELECT add_health FROM ancestries WHERE id = ?",
            character_info.ancestry_id
        )
        .fetch_one(db),
        sqlx::query_scalar!(
            "SELECT add_health FROM levels WHERE level = 1 AND path_id = ?",
            character_info.novice_path_id
        )
        .fetch_one(db),
    );

    let init_health = path_health? + ancestry_health?.unwrap_or(0);

    let record = sqlx::query!(
        "INSERT INTO characters (
            name,
            level,
            health,
            damage,
            strength,
            agility,
            intellect,
            will,
            profession_id,
            ancestry_id,
            novice_path_id
        ) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        character_info.name,
        1,
        init_health,
        init_health,
        character_info.strength,
        character_info.agility,
        character_info.intellect,
        character_info.will,
        character_info.profession_id,
        character_info.ancestry_id,
        character_info.novice_path_id
    )
    .execute(db)
    .await?;

    Ok(record.last_insert_rowid())
}

pub async fn get(db: &Pool<Sqlite>, id: i64) -> WWResult<FullCharacter> {
    let (raw_character, all_character_choices) = futures::join!(
        sqlx::query_as!(RawCharacter, "SELECT * FROM characters WHERE id = ?", id).fetch_one(db),
        character_choices::get_for_character(db.clone(), id),
    );

    let raw_character = raw_character?;
    let saved_character_choices = Arc::new(all_character_choices?);

    let (profession, ancestry, novice_path, expert_path, master_path) = futures::join!(
        professions::get(db, raw_character.profession_id),
        ancestries::get(db, raw_character.ancestry_id),
        paths::get(db, raw_character.novice_path_id),
        paths::get_from_opt(db, raw_character.expert_path_id),
        paths::get_from_opt(db, raw_character.master_path_id),
    );

    let profession = profession?;
    let ancestry = ancestry?;
    let novice_path = novice_path?;

    let mut fields = CumulativeFields::from_ancestry(&ancestry)?;

    for level in &novice_path.levels {
        if level.level <= raw_character.level {
            fields.process_level(level, &novice_path.name)?;
        }
    }
    let expert_path = match expert_path? {
        None => None,
        Some(path) => {
            for level in &path.levels {
                if level.level <= raw_character.level {
                    fields.process_level(level, &path.name)?;
                }
            }
            Some(path)
        }
    };
    let master_path = match master_path? {
        None => None,
        Some(path) => {
            for level in &path.levels {
                if level.level <= raw_character.level {
                    fields.process_level(level, &path.name)?;
                }
            }
            Some(path)
        }
    };

    let health = clamp(raw_character.health, 0, fields.max_health);
    let damage = clamp(raw_character.damage, 0, health);

    let dash_set = DashSet::new();
    let mut modifier_futures = FuturesUnordered::new();
    for choice in &fields.choices {
        let saved_choices = saved_character_choices.clone();
        let dash_set = dash_set.clone();
        let db = db.clone();
        modifier_futures.push(collect_from_modifier_tree(db, choice, saved_choices, dash_set));
    }
    let mut selections = ModifierSelections::new();
    while let Some(selection) = modifier_futures.next().await {
        selections.merge(selection?);
    }

    Ok(FullCharacter {
        id,
        name: raw_character.name,
        level: raw_character.level,
        health: health,
        damage: damage,
        strength: raw_character.strength,
        agility: raw_character.agility,
        intellect: raw_character.intellect,
        will: raw_character.will,
        profession,
        ancestry,
        novice_path: novice_path,
        expert_path: expert_path,
        master_path: master_path,
        created_at: raw_character.created_at.to_string(),

        // Fields derived from ancestries, paths, and choices
        max_health: fields.max_health,
        nat_def: fields.nat_def,
        arm_def: fields.arm_def,
        speed: fields.speed,
        bonus_dmg: fields.bonus_dmg,
        speed_traits: fields.speed_traits,
        senses: fields.senses,
        immunities: fields.immunities,
        languages: fields.languages,
        size: fields.size,
        // choices: fields.choices,
    })
}

struct CumulativeFields {
    max_health: i64,
    nat_def: i64,
    arm_def: i64,
    speed: i64,
    bonus_dmg: i64,
    speed_traits: Vec<FullSpeedTrait>,
    senses: Vec<FullSense>,
    immunities: Vec<Immunity>,
    languages: Vec<Language>,
    size: Size,
    choices: Vec<FullModifier>,
}

impl CumulativeFields {
    fn from_ancestry(ancestry: &FullAncestry) -> WWResult<Self> {
        let mut choices = Vec::new();
        for talent in &ancestry.talents {
            let mut these_mods = FullModifier::from_path_talent(talent)?;
            choices.append(&mut these_mods);
        }

        Ok(Self {
            max_health: ancestry.add_health,
            nat_def: ancestry.add_nat_def,
            speed: ancestry.speed,
            speed_traits: ancestry.speed_traits.clone(),
            senses: ancestry.senses.clone(),
            immunities: ancestry.immunities.clone(),
            languages: ancestry.languages.clone(),
            size: ancestry.size.clone(),
            arm_def: 0,
            bonus_dmg: 0,
            choices,
        })
    }

    fn process_level(&mut self, level: &FullLevel, path_name: &str) -> WWResult<()> {
        self.max_health += level.add_health;
        self.nat_def += level.add_nat_def;
        self.arm_def += level.add_arm_def;
        self.speed += level.add_speed;
        self.bonus_dmg += level.add_bonus_dmg;
        self.speed_traits.append(&mut level.speed_traits.clone());
        if let Some(size) = level.size.clone() {
            self.size = size
        };

        if level.lang_choices > 0 {
            let target = ChooseTarget::Language(level.lang_choices as u32);
            self.choices.push(level_choice(format!("Language;{};lvl{};", path_name, level.level), target));
        }

        if level.trad_choices > 0 {
            let target = ChooseTarget::Tradition(level.trad_choices as u32);
            self.choices.push(level_choice(format!("Tradition;{};lvl{};", path_name, level.level), target));
        }

        if level.novice_spells > 0 {
            let target = ChooseTarget::NoviceSpell(level.trad_choices as u32);
            self.choices.push(level_choice(format!("NoviceSpell;{};lvl{};", path_name, level.level), target));
        }

        if level.expert_spells > 0 {
            let target = ChooseTarget::ExpertSpell(level.trad_choices as u32);
            self.choices.push(level_choice(format!("ExpertSpell;{};lvl{};", path_name, level.level), target));
        }

        if level.master_spells > 0 {
            let target = ChooseTarget::MasterSpell(level.trad_choices as u32);
            self.choices.push(level_choice(format!("MasterSpell;{};lvl{};", path_name, level.level), target));
        }

        for talent in &level.path_talents {
            let mut these_mods = FullModifier::from_path_talent(talent)?;
            self.choices.append(&mut these_mods);
        }
        Ok(())
    }
}

fn clamp(value: i64, min: i64, max: i64) -> i64 {
    if value <= min {
        min
    } else if value >= max {
        max
    } else {
        value
    }
}

fn level_choice(path_str: String, choose_target: ChooseTarget) -> FullModifier {
    let target_strs = choose_target.choice_strings();
    FullModifier {
        path_str,
        mod_details: Modifier {
            when: WhenMod::Permanent,
            target: Target::Choose(
                choose_target,
                target_strs,
            ),
            condition: Condition::None,
        }
    }
}
