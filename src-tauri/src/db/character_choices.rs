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
    },
    mod_dsl::ast::{ChooseTarget, GrantTarget, LoseTarget, Modifier, OverrideTarget, Target},
    modifiers::{FullModifier, HasModifiers},
    WWError::Generic,
    WWResult,
};

const DISCLAIMER: &'static str = "If you're getting this error, this is a bug with the system. You did nothing wrong (probably).";

#[derive(Debug)]
struct RawCharacterChoice {
    id: i64,
    choice_key: String,
    selection: String,
    dismissable: Option<String>,
    duration: ChoiceDuration,
}

#[derive(Debug)]
pub struct CharacterChoice {
    id: i64,
    selection: String,
    dismissable: Option<String>,
    duration: ChoiceDuration,
}

pub async fn get_for_character(
    db: Pool<Sqlite>,
    id: i64,
) -> WWResult<HashMap<String, CharacterChoice>> {
    let choices = sqlx::query_as!(
        RawCharacterChoice,
        "SELECT id, choice_key, selection, dismissable, duration
        FROM character_choices WHERE character_id = ?",
        id
    )
    .fetch_all(&db)
    .await?;

    let mut choice_map = HashMap::new();
    for choice in choices {
        choice_map.insert(
            choice.choice_key,
            CharacterChoice {
                id: choice.id,
                selection: choice.selection,
                dismissable: choice.dismissable,
                duration: choice.duration,
            },
        );
    }

    Ok(choice_map)
}

// All data provided by Modifiers, and unselected Choices
#[derive(Debug)]
pub struct ModifierSelections {
    pub strength: i64,
    pub agility: i64,
    pub intellect: i64,
    pub will: i64,
    pub speed: i64,
    pub health: i64,
    pub defense: i64,
    pub nat_def: i64,
    pub bonus_damage: i64,
    pub gained_languages: Vec<String>,
    pub gained_traditions: Vec<String>,
    pub gained_senses: Vec<String>,
    pub gained_immunities: Vec<String>,
    pub gained_speed_traits: Vec<String>,
    pub statblock_overrides: Vec<(i64, String)>,
    pub gained_language_ids: Vec<i64>,
    pub gained_profession_ids: Vec<i64>,
    pub gained_spell_ids: Vec<i64>,
    pub gained_tradition_ids: Vec<i64>,
    pub modified_slots: Vec<SlotMod>,

    // These lists are for the source/name and tradition/name respectively, and are checked during the
    // flattening of the tree to prune gained_talents and gained_magic_talents respectively.
    pub lost_talents: Vec<(String, String)>,
    pub lost_magic_talents: Vec<(String, String)>,

    // This is how the ModifierSelections recurses into a tree structure. It will later be collapsed into
    // a flat structure, during which time the accumulated "lost" talents can prune the tree.
    pub gained_talents: Vec<(FullPathTalent, Vec<ModifierSelections>)>,
    pub gained_magic_talents: Vec<(FullMagicTalent, Vec<ModifierSelections>)>,

    pub required_choices: Vec<FullModifier>,
}

impl ModifierSelections {
    pub fn new() -> Self {
        Self {
            strength: 0,
            agility: 0,
            intellect: 0,
            will: 0,
            speed: 0,
            health: 0,
            defense: 0,
            nat_def: 0,
            bonus_damage: 0,
            gained_languages: Vec::new(),
            gained_traditions: Vec::new(),
            gained_senses: Vec::new(),
            gained_immunities: Vec::new(),
            gained_speed_traits: Vec::new(),
            gained_talents: Vec::new(),
            gained_magic_talents: Vec::new(),
            gained_language_ids: Vec::new(),
            gained_profession_ids: Vec::new(),
            gained_spell_ids: Vec::new(),
            gained_tradition_ids: Vec::new(),
            modified_slots: Vec::new(),
            statblock_overrides: Vec::new(),
            lost_talents: Vec::new(),
            lost_magic_talents: Vec::new(),
            required_choices: Vec::new(),
        }
    }

    pub fn merge(&mut self, mut other: Self) {
        self.strength += other.strength;
        self.agility += other.agility;
        self.intellect += other.intellect;
        self.will += other.will;
        self.speed += other.speed;
        self.health += other.health;
        self.defense += other.defense;
        self.nat_def += other.nat_def;
        self.bonus_damage += other.bonus_damage;

        self.gained_languages.append(&mut other.gained_languages);
        self.gained_traditions.append(&mut other.gained_traditions);
        self.gained_senses.append(&mut other.gained_senses);
        self.gained_immunities.append(&mut other.gained_immunities);
        self.gained_speed_traits
            .append(&mut other.gained_speed_traits);
        self.gained_talents.append(&mut other.gained_talents);
        self.gained_language_ids
            .append(&mut other.gained_language_ids);
        self.gained_profession_ids
            .append(&mut other.gained_profession_ids);
        self.gained_spell_ids.append(&mut other.gained_spell_ids);
        self.gained_tradition_ids
            .append(&mut other.gained_tradition_ids);
        self.modified_slots.append(&mut other.modified_slots);
        self.gained_magic_talents
            .append(&mut other.gained_magic_talents);
        self.statblock_overrides
            .append(&mut other.statblock_overrides);
        self.lost_talents.append(&mut other.lost_talents);
        self.lost_magic_talents
            .append(&mut other.lost_magic_talents);
        self.required_choices.append(&mut other.required_choices);
    }

    pub fn collect_losses(&self) -> Losses {
        let mut losses = Losses::new();

        for talent in &self.lost_talents {
            losses.path_talents.insert(talent.clone());
        }
        for (_, sub_trees) in &self.gained_talents {
            for sub_tree in sub_trees {
                losses.merge(sub_tree.collect_losses());
            }
        }

        for talent in &self.lost_magic_talents {
            losses.magic_talents.insert(talent.clone());
        }
        for (_, sub_trees) in &self.gained_magic_talents {
            for sub_tree in sub_trees {
                losses.merge(sub_tree.collect_losses());
            }
        }

        losses
    }

    pub fn collect(mut self, losses: &Losses) -> Self {
        let path_talents: Vec<(FullPathTalent, Vec<ModifierSelections>)> =
            mem::replace(&mut self.gained_talents, Vec::new())
                .into_iter()
                .filter(|(talent, _)| {
                    !losses
                        .path_talents
                        .contains(&(talent.source.to_owned(), talent.name.to_owned()))
                })
                .map(|(talent, sub_trees)| {
                    sub_trees
                        .into_iter()
                        .for_each(|sub_tree| self.merge(sub_tree.collect(&losses)));
                    (talent, Vec::new())
                })
                .collect();
        let magic_talents: Vec<(FullMagicTalent, Vec<ModifierSelections>)> =
            mem::replace(&mut self.gained_magic_talents, Vec::new())
                .into_iter()
                .filter(|(talent, _)| {
                    !losses
                        .magic_talents
                        .contains(&(talent.tradition_name.to_owned(), talent.name.to_owned()))
                })
                .map(|(talent, sub_trees)| {
                    sub_trees
                        .into_iter()
                        .for_each(|sub_tree| self.merge(sub_tree.collect(&losses)));
                    (talent, Vec::new())
                })
                .collect();

        self.gained_talents = path_talents;
        self.gained_magic_talents = magic_talents;
        self
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub enum SlotMod {
    Plus { amount: i64, spell_id: i64 },
    Times { amount: i64, spell_id: i64 },
}

pub struct Losses {
    path_talents: HashSet<(String, String)>,
    magic_talents: HashSet<(String, String)>,
}

impl Losses {
    pub fn new() -> Self {
        Self {
            path_talents: HashSet::new(),
            magic_talents: HashSet::new(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.path_talents.extend(other.path_talents.into_iter());
        self.magic_talents.extend(other.magic_talents.into_iter());
    }
}

// This function is totally nuts.
pub async fn collect_from_modifier_tree(
    db: Pool<Sqlite>,
    modifier: &FullModifier,

    // Only read from, never written to, so an Arc over a sync HashMap is enough
    saved_choices: Arc<HashMap<String, CharacterChoice>>,

    // Read from and written to in equal quantity, so an async-safe sharded HashSet is used
    processed_keys: DashSet<String>,
) -> WWResult<ModifierSelections> {
    let mut selections = ModifierSelections::new();
    let mut new_choice_mods: Vec<FullModifier> = Vec::new();

    let mut new_path_talents: Vec<(String, String)> = Vec::new();
    let mut new_magic_talents: Vec<(String, String)> = Vec::new();
    let mut new_magic_talent_ids: Vec<i64> = Vec::new();
    let mut new_choice_selections: Vec<i64> = Vec::new();

    match modifier.mod_details.target.clone() {
        Target::Grant(grant_target) => handle_grant_target(
            grant_target,
            &modifier,
            &mut selections,
            &mut new_choice_mods,
            &mut new_path_talents,
            &mut new_magic_talents,
        ),

        Target::Lose(lose_target) => match lose_target {
            LoseTarget::Talent(source, talent) => {
                selections.lost_talents.push((source, talent));
            }
            LoseTarget::MagicTalent(tradition, talent) => {
                selections.lost_magic_talents.push((tradition, talent));
            }
        },

        Target::Override(override_target) => match override_target {
            OverrideTarget::StatBlock(block_name) => {
                saved_choices
                    .get(&format!("Override;{block_name}"))
                    .map(|asdf| {
                        let idx = asdf.selection.parse::<i64>().unwrap_or(-1);
                        selections.statblock_overrides.push((idx, block_name))
                    });
            }
            _ => {}
        },

        Target::Choose(choose_target, _) => match choose_target {
            ChooseTarget::Score(_) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_score_choice,
                )?;
            }
            ChooseTarget::Language(_) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_id_choice,
                )?
                .map(|id| selections.gained_language_ids.push(id));
            }
            ChooseTarget::NoviceSpell(_)
            | ChooseTarget::NoviceSpellFrom(_, _)
            | ChooseTarget::ExpertSpell(_)
            | ChooseTarget::ExpertSpellFrom(_, _)
            | ChooseTarget::MasterSpell(_)
            | ChooseTarget::MasterSpellFrom(_, _) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_id_choice,
                )?
                .map(|id| selections.gained_spell_ids.push(id));
            }
            ChooseTarget::Profession(_) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_id_choice,
                )?
                .map(|id| selections.gained_profession_ids.push(id));
            }
            ChooseTarget::Select(_, _) | ChooseTarget::SelectAgain(_, _) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_id_choice,
                )?
                .map(|id| {
                    new_choice_selections.push(id);
                });
            }

            ChooseTarget::Tradition(_) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_twin_id_choice,
                )?
                .map(|(trad_id, talent_id)| {
                    selections.gained_tradition_ids.push(trad_id);
                    new_magic_talent_ids.push(talent_id);
                });
            }
            ChooseTarget::MagicTalent(_, _) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_id_choice,
                )?
                .map(|talent_id| {
                    new_magic_talent_ids.push(talent_id);
                });
            }

            ChooseTarget::Slots(_) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_slots_choice,
                )?
                .map(|slot| {
                    selections.modified_slots.push(slot);
                });
            }
        },

        // The Apply and remaining Override mods affect nothing and are only used on the frontend
        _ => {}
    }

    // Check any gained talents/choice selections for additional modifiers
    let (path_talents, magic_talents, more_magic_talents, choices) = futures::join!(
        path_talents::get_by_selections(db.clone(), new_path_talents),
        magic_talents::get_by_selections(db.clone(), new_magic_talents),
        magic_talents::get_by_ids(db.clone(), new_magic_talent_ids),
        choice_selections::get_for_choice_ids(&db, new_choice_selections),
    );

    let path_talents = path_talents?;
    let mut magic_talents = magic_talents?;
    magic_talents.append(&mut more_magic_talents?);
    let choices = choices?;

    let path_talents_with_sub_trees = futures::stream::iter(path_talents)
        .map(|talent| {
            let db = db.clone();
            let saved_choices = saved_choices.clone();
            let processed_keys = processed_keys.clone();
            async move { process_with_modifiers(talent, db, saved_choices, processed_keys).await }
        })
        .buffered(100)
        .try_collect();

    let magic_talents_with_sub_trees = futures::stream::iter(magic_talents)
        .map(|talent| {
            let db = db.clone();
            let saved_choices = saved_choices.clone();
            let processed_keys = processed_keys.clone();
            async move { process_with_modifiers(talent, db, saved_choices, processed_keys).await }
        })
        .buffered(100)
        .try_collect();

    let granted_choice_futures = futures::stream::iter(new_choice_mods)
        .map(|modifier| {
            let saved_choices_clone = saved_choices.clone();
            let processed_keys_clone = processed_keys.clone();
            let db = db.clone();
            async move {
                collect_from_modifier_tree(db, &modifier, saved_choices_clone, processed_keys_clone)
                    .await
            }
        })
        .buffered(20)
        .try_collect::<Vec<_>>();

    let choice_selection_sub_trees = futures::stream::iter(choices)
        .map(|choice| {
            let db = db.clone();
            let saved_choices = saved_choices.clone();
            let processed_keys = processed_keys.clone();
            async move { process_just_modifiers(choice, db, saved_choices, processed_keys).await }
        })
        .buffered(100)
        .try_collect::<Vec<_>>();

    let (path_talents, magic_talents, granted_choices, chosen_selections) = futures::join!(
        path_talents_with_sub_trees,
        magic_talents_with_sub_trees,
        granted_choice_futures,
        choice_selection_sub_trees,
    );

    // No sub-trees needed here
    for s in granted_choices? {
        selections.merge(s);
    }

    for selection in chosen_selections? {
        for s in selection {
            selections.merge(s);
        }
    }

    // After checking talent mods, they go into the selection collection.
    selections.gained_talents.append(&mut path_talents?);
    selections.gained_magic_talents.append(&mut magic_talents?);

    Ok(selections)
}

fn handle_grant_target(
    grant_target: GrantTarget,
    modifier: &FullModifier,
    selections: &mut ModifierSelections,
    new_choice_mods: &mut Vec<FullModifier>,
    new_path_talents: &mut Vec<(String, String)>,
    new_magic_talents: &mut Vec<(String, String)>,
) {
    match grant_target {
        GrantTarget::Str(x) => {
            selections.strength += x as i64;
        }
        GrantTarget::Agl(x) => {
            selections.agility += x as i64;
        }
        GrantTarget::Int(x) => {
            selections.intellect += x as i64;
        }
        GrantTarget::Will(x) => {
            selections.will += x as i64;
        }
        GrantTarget::Speed(x) => {
            selections.speed += x as i64;
        }
        GrantTarget::Health(x) => {
            selections.health += x as i64;
        }
        GrantTarget::Defense(x) => {
            selections.defense += x as i64;
        }
        GrantTarget::NatDef(x) => {
            selections.nat_def += x as i64;
        }
        GrantTarget::BonusDamage(x) => {
            selections.bonus_damage += x as i64;
        }
        GrantTarget::Language(mut items) => {
            selections.gained_languages.append(&mut items);
        }
        GrantTarget::Tradition(mut items) => {
            // Granted Traditions creates a new choice to pick a talent from that tradition
            for item in &items {
                let choose_target = ChooseTarget::MagicTalent(1, item.clone());
                let choice_strings = choose_target.choice_strings();
                new_choice_mods.push(FullModifier {
                    path_str: modifier.path_str.to_owned(),
                    mod_details: Modifier {
                        when: modifier.mod_details.when,
                        condition: modifier.mod_details.condition.clone(),
                        target: Target::Choose(choose_target, choice_strings),
                    },
                });
            }
            selections.gained_traditions.append(&mut items);
        }
        GrantTarget::Sense(mut items) => {
            selections.gained_senses.append(&mut items);
        }
        GrantTarget::Immunity(mut items) => {
            selections.gained_immunities.append(&mut items);
        }
        GrantTarget::SpeedTrait(mut items) => {
            selections.gained_speed_traits.append(&mut items);
        }
        GrantTarget::Talent(source, talent) => {
            new_path_talents.push((source, talent));
        }
        GrantTarget::MagicTalent(tradition, talent) => {
            new_magic_talents.push((tradition, talent));
        }
    }
}

fn handle_choice<F, T>(
    modifier: &FullModifier,
    saved_choices: Arc<HashMap<String, CharacterChoice>>,
    mut selections: &mut ModifierSelections,
    mut on_selection: F,
) -> WWResult<Option<T>>
where
    F: FnMut(&str, &mut ModifierSelections) -> WWResult<T>,
{
    let mut required_choice_strings = Vec::new();
    for (choice_key, full_key) in modifier.keys() {
        match saved_choices.get(&full_key) {
            Some(ch) => {
                return Ok(Some(on_selection(&ch.selection, &mut selections)?));
            }
            None => {
                if modifier.required() {
                    required_choice_strings.push(choice_key);
                }
            }
        }
    }
    if required_choice_strings.len() > 0 {
        let mut new_modifier = modifier.clone();
        new_modifier.set_choice_strings(required_choice_strings);
        selections.required_choices.push(new_modifier);
    }

    Ok(None)
}

fn parse_score_choice(entry: &str, selections: &mut ModifierSelections) -> WWResult<()> {
    match entry {
        "Strength" => selections.strength += 1,
        "Agility" => selections.agility += 1,
        "Intellect" => selections.intellect += 1,
        "Will" => selections.will += 1,
        _ => {
            return Err(Generic(format!(
                "Failed to parse Score choice selection '{entry}'"
            )))
        }
    }

    Ok(())
}

fn parse_id_choice(entry: &str, _: &mut ModifierSelections) -> WWResult<i64> {
    Ok(entry
        .parse()
        .map_err(|_| Generic(format!("Failed to parse ID value '{entry}'. {DISCLAIMER}")))?)
}

fn parse_twin_id_choice(entry: &str, _: &mut ModifierSelections) -> WWResult<(i64, i64)> {
    let mut twin_ids = entry.split('|');
    let trad_id = twin_ids
        .next()
        .ok_or_else(|| Generic(format!("Blank twin ID value '{entry}'. {DISCLAIMER}")))?
        .parse()
        .map_err(|_| Generic(format!("Failed to parse ID value '{entry}'. {DISCLAIMER}")))?;
    let talent_id = twin_ids
        .next()
        .ok_or_else(|| {
            Generic(format!(
                "Failed to parse twin ID value '{entry}'. {DISCLAIMER}"
            ))
        })?
        .parse()
        .map_err(|_| Generic(format!("Failed to parse ID value '{entry}'. {DISCLAIMER}")))?;

    Ok((trad_id, talent_id))
}

fn parse_slots_choice(entry: &str, _: &mut ModifierSelections) -> WWResult<SlotMod> {
    let make_error = || {
        Generic(format!(
            "Failed to parse slots decision '{entry}'. {DISCLAIMER}"
        ))
    };
    let mut parts = entry.split('|');
    let (kind, amount, id) = (parts.next(), parts.next(), parts.next());
    if kind.is_some() && amount.is_some() && id.is_some() {
        let kind = kind.unwrap();
        let amount: i64 = amount.unwrap().parse().map_err(|_| make_error())?;
        let spell_id: i64 = id.unwrap().parse().map_err(|_| make_error())?;
        match kind {
            "times" => Ok(SlotMod::Times { amount, spell_id }),
            "plus" => Ok(SlotMod::Plus { amount, spell_id }),
            _ => Err(make_error()),
        }
    } else {
        Err(make_error())
    }
}

async fn process_just_modifiers<T>(
    has_mods: T,
    db: Pool<Sqlite>,
    saved_choices: Arc<HashMap<String, CharacterChoice>>,
    processed_keys: DashSet<String>,
) -> WWResult<Vec<ModifierSelections>>
where
    T: HasModifiers,
{
    let db = db.clone();
    let saved_choices = saved_choices.clone();
    let modifiers = has_mods.modifiers();

    let sub_trees =
        futures::stream::iter(modifiers)
            .map(|modifier| {
                let saved_choices = saved_choices.clone();
                let processed_keys = processed_keys.clone();
                let db = db.clone();
                async move {
                    collect_from_modifier_tree(db, &modifier, saved_choices, processed_keys).await
                }
            })
            .buffered(100)
            .try_collect()
            .await?;

    Ok(sub_trees)
}

async fn process_with_modifiers<T>(
    has_mods: T,
    db: Pool<Sqlite>,
    saved_choices: Arc<HashMap<String, CharacterChoice>>,
    processed_keys: DashSet<String>,
) -> WWResult<(T, Vec<ModifierSelections>)>
where
    T: HasModifiers + Clone,
{
    let has_mods = has_mods.clone();
    let db = db.clone();
    let saved_choices = saved_choices.clone();
    let modifiers = has_mods.modifiers();

    let sub_trees =
        futures::stream::iter(modifiers)
            .map(|modifier| {
                let saved_choices = saved_choices.clone();
                let processed_keys = processed_keys.clone();
                let db = db.clone();
                async move {
                    collect_from_modifier_tree(db, &modifier, saved_choices, processed_keys).await
                }
            })
            .buffered(100)
            .try_collect()
            .await?;

    Ok((has_mods, sub_trees))
}
