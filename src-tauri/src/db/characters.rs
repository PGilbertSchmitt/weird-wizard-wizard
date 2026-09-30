use dashmap::DashSet;
use futures::{stream::FuturesUnordered, StreamExt};
use std::{sync::Arc, time::SystemTime};

use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, Pool, Sqlite};
use ts_rs::TS;

use crate::{
    WWError::Generic, WWResult, db::{
        ancestries::{self, FullAncestry}, character_choices::{self, ModifierSelections, SlotMod, collect_from_modifier_tree}, choice_selections::FullChoice, etc::Size, languages::{self, Language}, levels::FullLevel, magic_talents::FullMagicTalent, path_talents::FullPathTalent, paths::{self, FullPath}, professions::{self, Profession}, senses::{self, FullSense}, speed_traits::{self, FullSpeedTrait}, spells::FullSpell, traditions::{self, TraditionIndexItem},
    }, mod_dsl::ast::{ChooseTarget, Condition, Modifier, Target, WhenMod}, modifiers::{FullModifier, ModifierPathNode},
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
    traditions: Vec<(TraditionIndexItem, String)>,
    speed_traits: Vec<(FullSpeedTrait, String)>,
    senses: Vec<(FullSense, String)>,
    immunities: Vec<(String, String)>, // Immunities are so simple, we probably don't even need a table
    languages: Vec<(Language, String)>,
    size: Size,

    path_talents: Vec<(FullPathTalent, String)>,
    magic_talents: Vec<(FullMagicTalent, String)>,
    spells: Vec<(FullSpell, String)>,

    modified_slots: Vec<SlotMod>,
    required_choices: Vec<FullModifier>,
    selected_choices: Vec<(FullModifier, Vec<FullChoice>)>,
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

    let init_health = path_health? + ancestry_health?;

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
        0,
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
    println!("Starting get for character {id}");
    let start = SystemTime::now();
    let (raw_character, all_character_choices) = futures::join!(
        sqlx::query_as!(RawCharacter, "SELECT * FROM characters WHERE id = ?", id).fetch_one(db),
        character_choices::get_for_character(db.clone(), id),
    );

    let raw_character = raw_character?;
    let saved_character_choices = Arc::new(all_character_choices?);

    let (
        profession,
        ancestry,
        novice_path,
        expert_path,
        master_path,
        all_languages,
        all_speed_traits,
        all_senses,
        all_traditions,
    ) = futures::join!(
        professions::get(db, raw_character.profession_id),
        ancestries::get(db, raw_character.ancestry_id),
        paths::get(db, raw_character.novice_path_id),
        paths::get_from_opt(db, raw_character.expert_path_id),
        paths::get_from_opt(db, raw_character.master_path_id),
        languages::get_all(db),
        speed_traits::get_all(db),
        senses::get_all(db),
        traditions::get_index(db),
    );

    let profession = profession?;
    let ancestry = ancestry?;
    let novice_path = novice_path?;

    let all_languages = all_languages?;
    let all_speed_traits = all_speed_traits?;
    let all_senses = all_senses?;
    let all_traditions = all_traditions?;

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

    let processed_keys = DashSet::new();
    let mut modifier_futures = FuturesUnordered::new();
    for choice in &fields.choices {
        let saved_choices = saved_character_choices.clone();
        let processed_keys = processed_keys.clone();
        let db = db.clone();
        modifier_futures.push(collect_from_modifier_tree(
            db,
            choice,
            saved_choices,
            processed_keys,
        ));
    }
    let mut selections = ModifierSelections::new();
    while let Some(selection) = modifier_futures.next().await {
        selections.merge(selection?);
    }

    // Prune and flatten selections
    let all_losses = selections.collect_losses();
    let mut selections = selections.collect(&all_losses);

    // In the following code, you will see linear searches. Normally, this would give me stomach ulcers,
    // but the vecs containing the search spaces are so small that it's probably more efficient than
    // building HashMaps with a fast hash function. Maybe for fun, I'll benchmark for Traditions which
    // has 33 official options.

    for (name, source) in selections.gained_languages {
        if let Some(language) = all_languages.iter().find(|lang| lang.name == name) {
            fields.languages.push((language.clone(), source));
        } else {
            return Err(Generic(format!("Cannot find language with name {name}")));
        }
    }

    for (id, source) in selections.gained_language_ids {
        if let Some(language) = all_languages.iter().find(|lang| lang.id == id) {
            fields.languages.push((language.clone(), source));
        } else {
            return Err(Generic(format!("Cannot find language with id {id}")));
        }
    }

    for (name, amount, source) in selections.gained_speed_traits {
        if let Some(raw_trait) = all_speed_traits.iter().find(|st| st.name == name) {
            let speed_trait = FullSpeedTrait {
                id: raw_trait.id,
                name: name.clone(),
                description: raw_trait.description.clone(),
                unit: raw_trait.unit.clone(),
                amount: amount,
            };
            fields.speed_traits.push((speed_trait, source));
        } else {
            return Err(Generic(format!("Cannot find speed trait with name {name}")));
        }
    }

    for (name, amount, source) in selections.gained_senses {
        if let Some(raw_trait) = all_senses.iter().find(|st| st.name == name) {
            let speed_trait = FullSpeedTrait {
                id: raw_trait.id,
                name: name.clone(),
                description: raw_trait.description.clone(),
                unit: raw_trait.unit.clone(),
                amount: amount,
            };
            fields.speed_traits.push((speed_trait, source));
        } else {
            return Err(Generic(format!("Cannot find sense with name {name}")));
        }
    }

    for (name, source) in selections.gained_traditions {
        if let Some(tradition) = all_traditions.iter().find(|trad| trad.name == name) {
            fields.traditions.push((tradition.clone(), source));
        } else {
            return Err(Generic(format!("Cannot find tradition with name {name}")));
        }
    }

    for (id, source) in selections.gained_tradition_ids {
        if let Some(tradition) = all_traditions.iter().find(|trad| trad.id == id) {
            fields.traditions.push((tradition.clone(), source));
        } else {
            return Err(Generic(format!("Cannot find tradition with id {id}")));
        }
    }

    for (talent, _, source) in selections.gained_talents {
        fields.path_talents.push((talent, source));
    }

    let mut magic_talents = Vec::new();
    for (talent, _, source) in selections.gained_magic_talents {
        magic_talents.push((talent, source));
    }

    fields.immunities.append(&mut selections.gained_immunities);

    println!("Took {}ms", start.elapsed().unwrap().as_millis());

    Ok(FullCharacter {
        id: raw_character.id,
        name: raw_character.name,
        level: raw_character.level,
        health: health,
        damage: damage,
        strength: raw_character.strength + selections.strength,
        agility: raw_character.agility + selections.agility,
        intellect: raw_character.intellect + selections.intellect,
        will: raw_character.will + selections.will,
        profession,
        ancestry,
        novice_path: novice_path,
        expert_path: expert_path,
        master_path: master_path,
        created_at: raw_character.created_at.to_string(),

        // Fields derived from ancestries, paths, grants, and choices
        max_health: fields.max_health + selections.health,
        nat_def: fields.nat_def + selections.nat_def,
        arm_def: fields.arm_def + selections.defense, // Is this right?
        speed: fields.speed + selections.speed,
        bonus_dmg: fields.bonus_dmg + selections.bonus_damage,
        traditions: fields.traditions,
        speed_traits: fields.speed_traits,
        senses: fields.senses,
        immunities: fields.immunities,
        languages: fields.languages,
        size: fields.size,

        path_talents: fields.path_talents,
        magic_talents,
        spells: selections.gained_spells,

        modified_slots: selections.modified_slots,
        required_choices: selections.required_choices,
        selected_choices: selections.selected_choices,
    })
}

struct CumulativeFields {
    max_health: i64,
    nat_def: i64,
    arm_def: i64,
    speed: i64,
    bonus_dmg: i64,
    traditions: Vec<(TraditionIndexItem, String)>,
    speed_traits: Vec<(FullSpeedTrait, String)>,
    senses: Vec<(FullSense, String)>,
    immunities: Vec<(String, String)>,
    languages: Vec<(Language, String)>,
    path_talents: Vec<(FullPathTalent, String)>,
    size: Size,
    choices: Vec<FullModifier>,
}

impl CumulativeFields {
    fn from_ancestry(ancestry: &FullAncestry) -> WWResult<Self> {
        let mut choices = Vec::new();
        let mut path_talents = Vec::new();
        let ancestry_source = format!("{} Ancestry", ancestry.name);

        for talent in &ancestry.talents {
            choices.append(&mut talent.modifiers.clone());
            path_talents.push((talent.clone(), ancestry_source.clone()));
        }

        Ok(Self {
            max_health: ancestry.add_health,
            nat_def: ancestry.add_nat_def,
            speed: ancestry.speed,
            speed_traits: ancestry
                .speed_traits
                .clone()
                .into_iter()
                .map(|tr| (tr, ancestry_source.clone()))
                .collect(),
            senses: ancestry
                .senses
                .clone()
                .into_iter()
                .map(|sense| (sense, ancestry_source.clone()))
                .collect(),
            immunities: ancestry
                .immunities
                .clone()
                .into_iter()
                .map(|imm| (imm.name, ancestry_source.clone()))
                .collect(),
            languages: ancestry
                .languages
                .clone()
                .into_iter()
                .map(|lang| (lang, ancestry_source.clone()))
                .collect(),
            size: ancestry.size.clone(),
            arm_def: 0,
            bonus_dmg: 0,
            traditions: Vec::new(),
            path_talents,
            choices,
        })
    }

    fn process_level(&mut self, level: &FullLevel, path_name: &str) -> WWResult<()> {
        self.max_health += level.add_health;
        self.nat_def += level.add_nat_def;
        self.arm_def += level.add_arm_def;
        self.speed += level.add_speed;
        self.bonus_dmg += level.add_bonus_dmg;

        // All paths at level 3 get 2 score increases
        if level.level == 3 {
            self.choices.push(level_choice(
                ModifierPathNode::LevelScore { level: level.level },
                ChooseTarget::Score(2),
            ))
        }

        // All paths at level 7 get 3 score increases
        if level.level == 7 {
            self.choices.push(level_choice(
                ModifierPathNode::LevelScore { level: level.level },
                ChooseTarget::Score(3),
            ))
        }

        let level_source = format!("{path_name} level {}", level.level);
        self.speed_traits.append(
            &mut level
                .speed_traits
                .clone()
                .into_iter()
                .map(|tr| (tr, level_source.clone()))
                .collect(),
        );

        if let Some(size) = level.size.clone() {
            self.size = size
        };

        for language in &level.languages {
            self.languages
                .push((language.clone(), level_source.clone()));
        }

        for speed_trait in &level.speed_traits {
            self.speed_traits
                .push((speed_trait.clone(), level_source.clone()));
        }

        for tradition in &level.traditions {
            self.traditions
                .push((tradition.clone(), level_source.clone()));
            let target = ChooseTarget::MagicTalent(1, vec![tradition.name.clone()]);
            self.choices.push(level_choice(
                ModifierPathNode::LevelMagicTalent {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
            ));
        }

        for talent in &level.path_talents {
            self.choices.append(&mut talent.modifiers.clone());
            self.path_talents
                .push((talent.clone(), level_source.clone()));
        }

        if level.lang_choices > 0 {
            let target = ChooseTarget::Language(level.lang_choices as u32);
            self.choices.push(level_choice(
                ModifierPathNode::LevelLanguage {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
            ));
        }

        if level.trad_choices > 0 {
            let target = ChooseTarget::Tradition(level.trad_choices as u32);
            self.choices.push(level_choice(
                ModifierPathNode::LevelTradition {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
            ));
        }

        if level.novice_spells > 0 {
            let target = ChooseTarget::NoviceSpell(level.novice_spells as u32);
            self.choices.push(level_choice(
                ModifierPathNode::LevelNoviceSpell {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
            ));
        }

        if level.expert_spells > 0 {
            let target = ChooseTarget::ExpertSpell(level.expert_spells as u32);
            self.choices.push(level_choice(
                ModifierPathNode::LevelExpertSpell {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
            ));
        }

        if level.master_spells > 0 {
            let target = ChooseTarget::MasterSpell(level.master_spells as u32);
            self.choices.push(level_choice(
                ModifierPathNode::LevelMasterSpell {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
            ));
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

fn level_choice(path_str: ModifierPathNode, choose_target: ChooseTarget) -> FullModifier {
    let target_strs = choose_target.choice_strings();
    FullModifier {
        path_node: path_str,
        mod_details: Modifier {
            when: WhenMod::Permanent,
            target: Target::Choose(choose_target, target_strs),
            condition: Condition::None,
        },
    }
}

pub async fn update_level(db: &Pool<Sqlite>, id: i64, level: i64) -> WWResult<()> {
    // let character = sqlx::query_as!(RawCharacter, "SELECT * FROM characters WHERE id = ?", id).fetch_one(db).await?;

    let clamped_level = clamp(level, 1, 10);
    sqlx::query!(
        "UPDATE characters SET level = ? WHERE id = ?",
        clamped_level,
        id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update_health(db: &Pool<Sqlite>, id: i64, health: i64, damage: i64) -> WWResult<()> {
    sqlx::query!(
        "UPDATE characters SET health = ?, damage = ? WHERE id = ?",
        health,
        damage,
        id
    )
    .execute(db)
    .await?;

    Ok(())
}
