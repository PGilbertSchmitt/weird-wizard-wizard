use futures::{StreamExt, TryStreamExt};
use std::{collections::HashMap, sync::Arc};

use dashmap::DashSet;
use sqlx::{Pool, Sqlite};

use crate::{
    db::{
        etc::ChoiceDuration,
        magic_talents::{self, FullMagicTalent},
        path_talents::{self, FullPathTalent},
    },
    mod_dsl::ast::{ChooseTarget, GrantTarget, LoseTarget, Modifier, OverrideTarget, Target},
    modifiers::FullModifier,
    WWError, WWResult,
};

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
    strength: i32,
    aglility: i32,
    intellect: i32,
    will: i32,
    speed: i32,
    health: i32,
    defense: i32,
    nat_def: i32,
    bonus_damage: i32,
    gained_languages: Vec<String>,
    gained_language_ids: Vec<i32>,
    gained_traditions: Vec<String>,
    gained_senses: Vec<String>,
    gained_immunities: Vec<String>,
    gained_speed_traits: Vec<String>,
    statblock_overrides: Vec<(i32, String)>,

    // These lists are for the source/name and tradition/name respectively, and are checked during the
    // flattening of the tree to prune gained_talents and gained_magic_talents respectively.
    lost_talents: Vec<(String, String)>,
    lost_magic_talents: Vec<(String, String)>,

    // This is how the ModifierSelections recurses into a tree structure. It will later be collapsed into
    // a flat structure, during which time the accumulated "lost" talents can prune the tree.
    gained_talents: Vec<(FullPathTalent, ModifierSelections)>,
    gained_magic_talents: Vec<(FullMagicTalent, ModifierSelections)>,

    required_choices: Vec<FullModifier>,
}

impl ModifierSelections {
    pub fn new() -> Self {
        Self {
            strength: 0,
            aglility: 0,
            intellect: 0,
            will: 0,
            speed: 0,
            health: 0,
            defense: 0,
            nat_def: 0,
            bonus_damage: 0,
            gained_languages: Vec::new(),
            gained_language_ids: Vec::new(),
            gained_traditions: Vec::new(),
            gained_senses: Vec::new(),
            gained_immunities: Vec::new(),
            gained_speed_traits: Vec::new(),
            gained_talents: Vec::new(),
            gained_magic_talents: Vec::new(),
            statblock_overrides: Vec::new(),
            lost_talents: Vec::new(),
            lost_magic_talents: Vec::new(),
            required_choices: Vec::new(),
        }
    }

    pub fn merge(&mut self, mut other: Self) {
        self.strength += other.strength;
        self.aglility += other.aglility;
        self.intellect += other.intellect;
        self.will += other.will;
        self.speed += other.speed;
        self.health += other.health;
        self.defense += other.defense;
        self.nat_def += other.nat_def;
        self.bonus_damage += other.bonus_damage;
        self.gained_languages.append(&mut other.gained_languages);
        self.gained_language_ids
            .append(&mut other.gained_language_ids);
        self.gained_traditions.append(&mut other.gained_traditions);
        self.gained_senses.append(&mut other.gained_senses);
        self.gained_immunities.append(&mut other.gained_immunities);
        self.gained_speed_traits
            .append(&mut other.gained_speed_traits);
        self.gained_talents.append(&mut other.gained_talents);
        self.gained_magic_talents
            .append(&mut other.gained_magic_talents);
        self.statblock_overrides
            .append(&mut other.statblock_overrides);
        self.lost_talents.append(&mut other.lost_talents);
        self.lost_magic_talents
            .append(&mut other.lost_magic_talents);
        self.required_choices.append(&mut other.required_choices);
    }
}

// This function is totally nuts, and proof that I wouldn't dare let a
// clanker touch my precious, handcrafted, brain-to-table codeslop.
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
                        let idx = asdf.selection.parse::<i32>().unwrap_or(-1);
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
                    parse_id_choice,
                )?;
            }
            ChooseTarget::Language(_) => {
                handle_choice(
                    &modifier,
                    saved_choices.clone(),
                    &mut selections,
                    parse_score_choice,
                )?;
            }

            _ => {}
        },

        // The Apply and remaining Override mods affect nothing and are only used on the frontend
        _ => {}
    }

    // Check any gained talents for additional modifiers
    let (path_talents, magic_talents) = futures::join!(
        get_selection_path_talents(db.clone(), &new_path_talents),
        get_selection_magic_talents(db.clone(), &new_magic_talents),
    );

    let path_talents = path_talents?;
    let magic_talents = magic_talents?;

    let path_talent_nodes: WWResult<Vec<_>> =
        path_talents
            .into_iter()
            .try_fold(Vec::new(), |mut pairs, talent| {
                pairs.extend(
                    FullModifier::from_path_talent(&talent)?
                        .into_iter()
                        .map(|new_mod| (talent.clone(), new_mod)),
                );
                Ok(pairs)
            });

    let magic_talent_nodes: WWResult<Vec<_>> =
        magic_talents
            .into_iter()
            .try_fold(Vec::new(), |mut pairs, talent| {
                pairs.extend(
                    FullModifier::from_magic_talent(&talent)?
                        .into_iter()
                        .map(|new_mod| (talent.clone(), new_mod)),
                );
                Ok(pairs)
            });

    let path_talent_futures = futures::stream::iter(path_talent_nodes?)
        .map(|(talent, modifier)| {
            let saved_choices = saved_choices.clone();
            let processed_keys = processed_keys.clone();
            let db = db.clone();
            async move {
                match collect_from_modifier_tree(db, &modifier, saved_choices, processed_keys).await
                {
                    Ok(sub_tree) => Ok((talent, sub_tree)),
                    Err(err) => Err(err),
                }
            }
        })
        .buffered(10)
        .try_collect::<Vec<_>>();

    let magic_talent_futures = futures::stream::iter(magic_talent_nodes?)
        .map(|(talent, modifier)| {
            let saved_choices = saved_choices.clone();
            let processed_keys = processed_keys.clone();
            let db = db.clone();
            async move {
                match collect_from_modifier_tree(db, &modifier, saved_choices, processed_keys).await
                {
                    Ok(sub_tree) => Ok((talent, sub_tree)),
                    Err(err) => Err(err),
                }
            }
        })
        .buffered(10)
        .try_collect::<Vec<_>>();

    // Recurse on all newly acquired mods
    let next_selection_futures = futures::stream::iter(new_choice_mods)
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

    let (path_talents, magic_talents, next_selections) = futures::join!(
        path_talent_futures,
        magic_talent_futures,
        next_selection_futures,
    );

    for s in next_selections? {
        selections.merge(s);
    }

    // After checking talent mods, they go into the selection collection.
    selections.gained_talents.append(&mut path_talents?);
    selections.gained_magic_talents.append(&mut magic_talents?);

    Ok(selections)
}

async fn get_selection_path_talents(
    db: Pool<Sqlite>,
    selections: &Vec<(String, String)>,
) -> WWResult<Vec<FullPathTalent>> {
    let talents: Vec<FullPathTalent> = futures::stream::iter(selections.iter())
        .map(|(source, talent_name)| {
            let db = db.clone();
            async move { path_talents::get_by_name_and_source(&db, talent_name, source).await }
        })
        .buffered(100)
        .try_collect()
        .await?;
    Ok(talents)
}

async fn get_selection_magic_talents(
    db: Pool<Sqlite>,
    selections: &Vec<(String, String)>,
) -> WWResult<Vec<FullMagicTalent>> {
    let talents: Vec<FullMagicTalent> = futures::stream::iter(selections.iter())
        .map(|(tradition_name, talent_name)| {
            let db = db.clone();
            async move {
                magic_talents::get_by_name_and_tradition(&db, tradition_name, talent_name).await
            }
        })
        .buffered(100)
        .try_collect()
        .await?;
    Ok(talents)
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
            selections.strength += x;
        }
        GrantTarget::Agl(x) => {
            selections.aglility += x;
        }
        GrantTarget::Int(x) => {
            selections.intellect += x;
        }
        GrantTarget::Will(x) => {
            selections.will += x;
        }
        GrantTarget::Speed(x) => {
            selections.speed += x;
        }
        GrantTarget::Health(x) => {
            selections.health += x;
        }
        GrantTarget::Defense(x) => {
            selections.defense += x;
        }
        GrantTarget::NatDef(x) => {
            selections.nat_def += x;
        }
        GrantTarget::BonusDamage(x) => {
            selections.bonus_damage += x;
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

fn handle_choice<F>(
    modifier: &FullModifier,
    saved_choices: Arc<HashMap<String, CharacterChoice>>,
    mut selections: &mut ModifierSelections,
    mut on_selection: F,
) -> WWResult<()>
where
    F: FnMut(&str, &mut ModifierSelections) -> WWResult<()>,
{
    for key in modifier.keys() {
        match saved_choices.get(&key) {
            Some(ch) => {
                on_selection(&ch.selection, &mut selections)?;
            }
            None => {
                if modifier.required() {
                    selections.required_choices.push(modifier.clone())
                }
            }
        }
    }

    Ok(())
}

fn parse_score_choice(entry: &str, selections: &mut ModifierSelections) -> WWResult<()> {
    match entry {
        "Strength" => selections.strength += 1,
        "Agility" => selections.aglility += 1,
        "Intellect" => selections.intellect += 1,
        "Will" => selections.will += 1,
        _ => {
            return Err(WWError::Generic(format!(
                "Failed to parse Score choice selection '{entry}'"
            )))
        }
    }

    Ok(())
}

fn parse_id_choice(entry: &str, selections: &mut ModifierSelections) -> WWResult<()> {
    let id = entry.parse().map_err(|_| WWError::Generic(String::new()))?;
    selections.gained_language_ids.push(id);
    Ok(())
}

fn parse_int(value: Option<&str>) -> WWResult<i32> {
    match value {
        Some(inner) => Ok(inner
            .parse::<i32>()
            .map_err(|_| WWError::Generic(String::new()))?),
        None => Err(WWError::Generic(String::new())),
    }
}
