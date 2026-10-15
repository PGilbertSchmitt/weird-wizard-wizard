use priority_queue::PriorityQueue;
use std::{collections::HashSet, mem, time::SystemTime};

use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, Pool, Sqlite};
use ts_rs::TS;

use crate::{
    WWResult, db::{
        ancestries::{self, FullAncestry}, character_effects::{self, CharacterEffect}, collect_effects::{
            Losses, ModifierSelections, SlotMod, modifier_tree_first_pass, modifier_tree_second_pass,
        }, collection_priority::{CollectionPriority as Priority, PriorityItem as PItem}, etc::{PathKind, Size}, languages, levels::FullLevel, paths::{self, FullPath}, professions::{self, Profession}, senses, speed_traits,
    }, mod_dsl::ast::{ChooseTarget, Condition, Modifier, Target, WhenMod}, modifiers::{FullModifier, ModifierPathNode}, selection_types::{
        GainedImmunity, GainedLanguage, GainedMagicTalent, GainedPathTalent, GainedProfession,
        GainedSense, GainedSpeedTrait, GainedSpell, GainedTradition,
    }, util::clamp,
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
pub struct RawCharacter {
    pub id: i64,
    pub name: String,
    pub level: i64,
    pub health: i64,
    pub damage: i64,
    pub strength: i64,
    pub agility: i64,
    pub intellect: i64,
    pub will: i64,
    pub profession_id: i64,
    pub ancestry_id: i64,
    pub novice_path_id: i64,
    pub expert_path_id: Option<i64>,
    pub master_path_id: Option<i64>,
    pub current_hour: i64,
    pub created_at: NaiveDateTime,
}

#[derive(TS, Debug, Serialize, Deserialize)]
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
    current_hour: i64,
    created_at: String,

    // Fields derived from ancestries, paths, and choices
    max_health: i64,
    nat_def: i64,
    arm_def: i64,
    speed: i64,
    bonus_dmg: i64,
    gained_professions: Vec<GainedProfession>,
    traditions: Vec<GainedTradition>,
    speed_traits: Vec<GainedSpeedTrait>,
    senses: Vec<GainedSense>,
    immunities: Vec<GainedImmunity>, // Immunities are so simple, we probably don't even need a table
    languages: Vec<GainedLanguage>,
    size: Size,

    path_talents: Vec<GainedPathTalent>,
    magic_talents: Vec<GainedMagicTalent>,
    spells: Vec<GainedSpell>,

    modified_slots: Vec<SlotMod>,
    required_choices: Vec<FullModifier>,
    selected_choices: Vec<(FullModifier, Vec<CharacterEffect>)>,
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
            novice_path_id,
            current_hour
        ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",
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
        character_info.novice_path_id,
        0
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
        character_effects::get_for_character(db, id),
    );

    let raw_character = raw_character?;
    let effect_cache = all_character_choices?;

    let (
        profession,
        ancestry,
        novice_path,
        expert_path,
        master_path,
        all_languages,
        all_speed_traits,
        all_senses,
    ) = futures::join!(
        professions::get(db, raw_character.profession_id),
        ancestries::get(db, raw_character.ancestry_id),
        paths::get(db, raw_character.novice_path_id),
        paths::get_from_opt(db, raw_character.expert_path_id),
        paths::get_from_opt(db, raw_character.master_path_id),
        languages::get_all(db),
        speed_traits::get_all(db),
        senses::get_all(db),
    );

    let profession = profession?;
    let ancestry = ancestry?;
    let novice_path = novice_path?;

    let language_cache = all_languages?;
    let speed_trait_cache = all_speed_traits?;
    let sense_cache = all_senses?;

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

    let mut losses = Losses::new();
    let field_modifiers = mem::replace(&mut fields.modifiers, Vec::new());
    for field_modifier in &field_modifiers {
        losses.merge(modifier_tree_first_pass(db.clone(), field_modifier.to_modifier()).await?);
    }

    let mut modifier_queue = PriorityQueue::new();
    for modifier in field_modifiers {
        match modifier {
            FieldModifier::PathTalent {
                source,
                name,
                modifier,
                priority,
            } => {
                if !losses.path_talents.contains(&(source, name)) {
                    modifier_queue.push(modifier, priority);
                }
            }
            FieldModifier::Other(modifier, priority) => {
                modifier_queue.push(modifier, priority);
            }
        }
    }
    
    // TODO: Delete all effects for character not found in this dashset
    let mut processed_effects = HashSet::new();
    let size = fields.size.clone();
    let mut selections = ModifierSelections::new(&raw_character, fields);

    while let Some((modifier, priority)) = modifier_queue.pop() {
        modifier_tree_second_pass(
            db.clone(),
            modifier,
            &priority,
            &mut modifier_queue,
            &mut selections,
            &mut processed_effects,
            &losses,
            &effect_cache,
            &language_cache,
            &speed_trait_cache,
            &sense_cache,
        )
        .await?;
    }

    let health = clamp(raw_character.health, 0, selections.health);
    let damage = clamp(raw_character.damage, 0, health);

    println!("Took {}ms", start.elapsed().unwrap().as_millis());

    Ok(FullCharacter {
        id: raw_character.id,
        name: raw_character.name,
        level: raw_character.level,
        health: health,
        damage: damage,
        strength: selections.strength,
        agility: selections.agility,
        intellect: selections.intellect,
        will: selections.will,
        profession,
        ancestry,
        novice_path: novice_path,
        expert_path: expert_path,
        master_path: master_path,
        current_hour: raw_character.current_hour,
        created_at: raw_character.created_at.to_string(),

        // Fields derived from ancestries, paths, grants, and choices
        max_health: selections.health,
        nat_def: selections.nat_def,
        arm_def: selections.arm_def,
        speed: selections.speed,
        bonus_dmg: selections.bonus_damage,
        gained_professions: selections.gained_professions,
        traditions: selections.gained_traditions,
        speed_traits: selections.gained_speed_traits,
        senses: selections.gained_senses,
        immunities: selections.gained_immunities,
        languages: selections.gained_languages,
        size,

        path_talents: selections.gained_path_talents,
        magic_talents: selections.gained_magic_talents,
        spells: selections.gained_spells,

        modified_slots: selections.modified_slots,
        required_choices: selections.required_choices,
        selected_choices: selections.selected_choices,
    })
}

// This lets us filter out lost path talents after the first pass. Magic talents
// are only acquired via grants/choices, so those have to be filtered by the
// second pass.
#[derive(Clone)]
pub enum FieldModifier {
    PathTalent {
        source: String,
        name: String,
        modifier: FullModifier,
        priority: Priority,
    },
    Other(FullModifier, Priority),
}

impl FieldModifier {
    fn to_modifier(&self) -> FullModifier {
        match self {
            Self::PathTalent { modifier, .. } => modifier.clone(),
            Self::Other(modifier, _) => modifier.clone(),
        }
    }
}

pub struct CumulativeFields {
    pub max_health: i64,
    pub nat_def: i64,
    pub arm_def: i64,
    pub speed: i64,
    pub bonus_dmg: i64,
    pub traditions: Vec<GainedTradition>,
    pub speed_traits: Vec<GainedSpeedTrait>,
    pub senses: Vec<GainedSense>,
    pub immunities: Vec<GainedImmunity>,
    pub languages: Vec<GainedLanguage>,
    pub path_talents: Vec<GainedPathTalent>,
    pub size: Size,
    pub modifiers: Vec<FieldModifier>,
}

impl CumulativeFields {
    fn from_ancestry(ancestry: &FullAncestry) -> WWResult<Self> {
        let mut modifiers = Vec::new();
        let mut path_talents = Vec::new();
        let ancestry_source = format!("{} Ancestry", ancestry.name);

        for talent in &ancestry.talents {
            for modifier in talent.modifiers.clone() {
                modifiers.push(FieldModifier::PathTalent {
                    source: talent.source.clone(),
                    name: talent.name.clone(),
                    modifier,
                    priority: Priority::new(0, PItem::PathTalent),
                });
            }
            path_talents.push(GainedPathTalent {
                talent: talent.clone(),
                source: ancestry_source.clone(),
                cast_hour: -1,
            });
        }

        Ok(Self {
            max_health: ancestry.add_health,
            nat_def: ancestry.add_nat_def,
            speed: ancestry.speed,
            speed_traits: ancestry
                .speed_traits
                .clone()
                .into_iter()
                .map(|speed_trait| GainedSpeedTrait {
                    speed_trait,
                    source: ancestry_source.clone(),
                    cast_hour: -1,
                })
                .collect(),
            senses: ancestry
                .senses
                .clone()
                .into_iter()
                .map(|sense| GainedSense {
                    sense,
                    source: ancestry_source.clone(),
                    cast_hour: -1,
                })
                .collect(),
            immunities: ancestry
                .immunities
                .clone()
                .into_iter()
                .map(|imm| GainedImmunity {
                    name: imm.name,
                    source: ancestry_source.clone(),
                    cast_hour: -1,
                })
                .collect(),
            languages: ancestry
                .languages
                .clone()
                .into_iter()
                .map(|language| GainedLanguage {
                    language,
                    source: ancestry_source.clone(),
                    cast_hour: -1,
                })
                .collect(),
            size: ancestry.size.clone(),
            arm_def: 0,
            bonus_dmg: 0,
            traditions: Vec::new(),
            path_talents,
            modifiers,
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
            self.modifiers.push(
                FieldModifier::Other(level_choice(
                    ModifierPathNode::LevelScore { level: level.level },
                    ChooseTarget::Score(2),
                    0,
                ), Priority::new(3, PItem::Score)),
            );
        }

        // All paths at level 7 get 3 score increases
        if level.level == 7 {
            self.modifiers.push(
                FieldModifier::Other(level_choice(
                    ModifierPathNode::LevelScore { level: level.level },
                    ChooseTarget::Score(3),
                    0,
                ), Priority::new(7, PItem::Score))
            );
        }

        let level_source = format!("{path_name} level {}", level.level);

        if let Some(size) = level.size.clone() {
            self.size = size
        };

        for language in &level.languages {
            self.languages.push(GainedLanguage {
                language: language.clone(),
                source: level_source.clone(),
                cast_hour: -1,
            });
        }

        for speed_trait in &level.speed_traits {
            self.speed_traits.push(GainedSpeedTrait {
                speed_trait: speed_trait.clone(),
                source: level_source.clone(),
                cast_hour: -1,
            });
        }

        for (idx, tradition) in level.traditions.iter().enumerate() {
            self.traditions.push(GainedTradition {
                tradition: tradition.clone(),
                source: level_source.clone(),
                cast_hour: -1,
            });

            let target = ChooseTarget::MagicTalent(1, vec![tradition.name.clone()]);
            self.modifiers.push(FieldModifier::Other(level_choice(
                ModifierPathNode::LevelMagicTalent {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
                idx,
            ), Priority::new(level.level, PItem::TraditionTalent)));
        }

        for talent in &level.path_talents {
            for modifier in talent.modifiers.clone() {
                self.modifiers.push(FieldModifier::PathTalent {
                    source: talent.source.clone(),
                    name: talent.name.clone(),
                    modifier,
                    priority: Priority::new(level.level, PItem::PathTalent),
                })
            }
            self.path_talents.push(GainedPathTalent {
                talent: talent.clone(),
                source: level_source.clone(),
                cast_hour: -1,
            });
        }

        if level.lang_choices > 0 {
            let target = ChooseTarget::Language(level.lang_choices as u32);
            self.modifiers.push(FieldModifier::Other(level_choice(
                ModifierPathNode::LevelLanguage {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
                0,
            ), Priority::new(level.level, PItem::Language)));
        }

        if level.trad_choices > 0 {
            let target = ChooseTarget::Tradition(level.trad_choices as u32);
            self.modifiers.push(FieldModifier::Other(level_choice(
                ModifierPathNode::LevelTradition {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
                0,
            ), Priority::new(level.level, PItem::PickTradition)));
        }

        if level.novice_spells > 0 {
            let target = ChooseTarget::NoviceSpell(level.novice_spells as u32);
            self.modifiers.push(FieldModifier::Other(level_choice(
                ModifierPathNode::LevelNoviceSpell {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
                0,
            ), Priority::new(level.level, PItem::Spell)));
        }

        if level.expert_spells > 0 {
            let target = ChooseTarget::ExpertSpell(level.expert_spells as u32);
            self.modifiers.push(FieldModifier::Other(level_choice(
                ModifierPathNode::LevelExpertSpell {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
                0,
            ), Priority::new(level.level, PItem::Spell)));
        }

        if level.master_spells > 0 {
            let target = ChooseTarget::MasterSpell(level.master_spells as u32);
            self.modifiers.push(FieldModifier::Other(level_choice(
                ModifierPathNode::LevelMasterSpell {
                    path_name: path_name.to_string(),
                    level: level.level,
                },
                target,
                0,
            ), Priority::new(level.level, PItem::Spell)));
        }

        Ok(())
    }
}

fn level_choice(
    path_str: ModifierPathNode,
    choose_target: ChooseTarget,
    idx: usize,
) -> FullModifier {
    FullModifier {
        path_node: path_str,
        mod_details: Modifier {
            idx,
            when: WhenMod::Permanent,
            target: Target::Choose(choose_target),
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

pub async fn set_path(db: &Pool<Sqlite>, id: i64, path_id: i64) -> WWResult<()> {
    let path_kind: PathKind =
        sqlx::query_scalar!("SELECT path_kind FROM paths WHERE id = ?", path_id)
            .fetch_one(db)
            .await?
            .into();

    match path_kind {
        PathKind::Novice => {
            sqlx::query!(
                "UPDATE characters SET novice_path_id = ? WHERE id = ?",
                path_id,
                id
            )
            .execute(db)
            .await?;
        }
        PathKind::Expert => {
            sqlx::query!(
                "UPDATE characters SET expert_path_id = ? WHERE id = ?",
                path_id,
                id
            )
            .execute(db)
            .await?;
        }
        PathKind::Master => {
            sqlx::query!(
                "UPDATE characters SET master_path_id = ? WHERE id = ?",
                path_id,
                id
            )
            .execute(db)
            .await?;
        }
    }

    Ok(())
}
