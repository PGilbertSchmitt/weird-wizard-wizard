use futures::{StreamExt, TryStreamExt};
use std::{collections::HashMap, sync::Arc};

use dashmap::DashSet;
use sqlx::{Pool, Sqlite};

use crate::{WWResult, db::{etc::ChoiceDuration, magic_talents::{self, FullMagicTalent}, path_talents::{self, FullPathTalent}}, mod_dsl::ast::{ChooseTarget, GrantTarget, Modifier, Target}, modifiers::FullModifier};

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

pub async fn get_for_character(db: &Pool<Sqlite>, id: i64) -> WWResult<HashMap<String, CharacterChoice>> {
    let choices = sqlx::query_as!(
        RawCharacterChoice,
        "SELECT id, choice_key, selection, dismissable, duration
        FROM character_choices WHERE character_id = ?",
        id
    ).fetch_all(db).await?;

    let mut choice_map = HashMap::new();
    for choice in choices {
        choice_map.insert(choice.choice_key, CharacterChoice {
            id: choice.id,
            selection: choice.selection,
            dismissable: choice.dismissable,
            duration: choice.duration,
        });
    };

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
    gained_traditions: Vec<String>,
    gained_senses: Vec<String>,
    gained_immunities: Vec<String>,
    gained_speed_traits: Vec<String>,
    gained_talents: Vec<FullPathTalent>,
    gained_magic_talents: Vec<FullMagicTalent>,
    // Figuring out how to prune the tree as we build it is tough.
    // How slow will this process be if we build the tree twice?:
    // - First to only collect LOSE modifiers
    // - Then again to process normally while not following LOSE trees
    // Alternatively, maintain the tree structure when building. Then prune it after construction,
    // then flatten at the end.
    // lost_talents: Vec<>
    // lost_magic_talents: Vec<>
}

impl ModifierSelections {
    fn new() -> Self {
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
            gained_traditions: Vec::new(),
            gained_senses: Vec::new(),
            gained_immunities: Vec::new(),
            gained_speed_traits: Vec::new(),
            gained_talents: Vec::new(),
            gained_magic_talents: Vec::new(),
        }
    }
    
    fn merge(&mut self, mut other: Self) {
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
        self.gained_traditions.append(&mut other.gained_traditions);
        self.gained_senses.append(&mut other.gained_senses);
        self.gained_immunities.append(&mut other.gained_immunities);
        self.gained_speed_traits.append(&mut other.gained_speed_traits);
        self.gained_talents.append(&mut other.gained_talents);
        self.gained_magic_talents.append(&mut other.gained_magic_talents);
    }
}

// This function is totally nuts, and proof that I wouldn't dare let a
// clanker touch my precious, handcrafted, farm-to-table slop.
pub async fn collect_from_modifier_tree(
    db: &Pool<Sqlite>,
    modifier: FullModifier,

    // Only read read, never written to, so an Arc over a sync HashMap is enough
    saved_choices: Arc<HashMap<String, CharacterChoice>>,

    // Read from and written to in equal quantity, so an async-safe sharded HashSet
    processed_keys: DashSet<String>,
) -> WWResult<ModifierSelections> {
    let mut selections = ModifierSelections::new();
    let mut new_choice_mods: Vec<FullModifier> = Vec::new();
    let key = modifier.path_str;

    let mut new_path_talents: Vec<(String, String)> = Vec::new();
    let mut new_magic_talents: Vec<(String, String)> = Vec::new();
    
    match modifier.mod_details.target {
        Target::Grant(grant) => {
            match grant {
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
                            path_str: key.clone(),
                            mod_details: Modifier {
                                when: modifier.mod_details.when,
                                condition: modifier.mod_details.condition.clone(),
                                target: Target::Choose(choose_target, choice_strings)
                            }
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

        _ => {}
    }

    // Check any gained talents for additional modifiers
    let (path_talents, magic_talents) = futures::join!(
        get_selection_path_talents(db, &new_path_talents),
        get_selection_magic_talents(db, &new_magic_talents),
    );

    let mut path_talents = path_talents?;
    let mut magic_talents = magic_talents?;

    for talent in &path_talents {
        new_choice_mods.append(&mut FullModifier::from_path_talent(talent)?);
    }
    for talent in &magic_talents {
        new_choice_mods.append(&mut FullModifier::from_magic_talent(talent)?);
    }

    // After checking talent mods, they go into the selection collection.
    selections.gained_talents.append(&mut path_talents);
    selections.gained_magic_talents.append(&mut magic_talents);

    // Recurse on all newly acquired mods
    let next_selections: Vec<ModifierSelections> = futures::stream::iter(new_choice_mods)
        .map(|modifier| {
            let saved_choices_clone = saved_choices.clone();
            let processed_keys_clone = processed_keys.clone();
            async move {
                collect_from_modifier_tree(db, modifier, saved_choices_clone, processed_keys_clone).await
            }
        })
        .buffered(100)
        .try_collect()
        .await?;

    for s in next_selections {
        selections.merge(s);
    }

    Ok(selections)
}

async fn get_selection_path_talents(db: &Pool<Sqlite>, selections: &Vec<(String, String)>) -> WWResult<Vec<FullPathTalent>> {
    let talents: Vec<FullPathTalent> = futures::stream::iter(selections.iter())
        .map(|(source, talent_name)| async move {
            path_talents::get_by_name_and_source(db, talent_name, source).await
        })
        .buffered(100)
        .try_collect()
        .await?;
    Ok(talents)
}

async fn get_selection_magic_talents(db: &Pool<Sqlite>, selections: &Vec<(String, String)>) -> WWResult<Vec<FullMagicTalent>> {
    let talents: Vec<FullMagicTalent> = futures::stream::iter(selections.iter())
        .map(|(tradition_name, talent_name)| async move {
            magic_talents::get_by_name_and_tradition(db, tradition_name, talent_name).await
        })
        .buffered(100)
        .try_collect()
        .await?;
    Ok(talents)
}
