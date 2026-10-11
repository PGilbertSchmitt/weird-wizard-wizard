use std::{
    collections::{HashMap, HashSet},
    future::Future,
    mem,
    pin::Pin,
    sync::Arc,
};

use dashmap::{DashMap, DashSet};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use ts_rs::TS;

use crate::{
    WWError::Generic, WWResult, db::{
        character_effects::CharacterEffect, magic_talents::{self, FullMagicTalent}, path_talents::{self, FullPathTalent}, professions, spells::{self, FullSpell},
    }, mod_dsl::ast::{
        ChooseTarget, Condition, GrantTarget, LoseTarget, Modifier, Target, WhenDuration,
    }, modifiers::FullModifier, selection_types::{
        GainedImmunity, GainedLanguage, GainedLanguageId, GainedProfession, GainedSense, GainedSpeedTrait, GainedTradition,
    },
};

const DISCLAIMER: &'static str = "If you're getting this error, this is a bug with the system. You did nothing wrong (probably).";

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
    pub gained_languages: Vec<GainedLanguage>,
    pub gained_immunities: Vec<GainedImmunity>,
    pub gained_traditions: Vec<GainedTradition>,
    pub gained_senses: Vec<GainedSense>,
    pub gained_speed_traits: Vec<GainedSpeedTrait>,
    pub gained_language_ids: Vec<GainedLanguageId>,
    pub gained_tradition_ids: Vec<(i64, String)>,
    pub gained_professions: Vec<GainedProfession>,
    pub statblock_overrides: Vec<(i64, String)>,
    pub modified_slots: Vec<SlotMod>,

    // These lists are for the source/name and tradition/name respectively, and are checked during the
    // flattening of the tree to prune gained_talents and gained_magic_talents respectively.
    pub lost_talents: Vec<(String, String)>,
    pub lost_magic_talents: Vec<(String, String)>,

    // This is how the ModifierSelections recurses into a tree structure. It will later be collapsed into
    // a flat structure, during which time the accumulated "lost" talents can prune the tree.
    pub gained_talents: Vec<(FullPathTalent, Vec<ModifierSelections>, String)>,
    pub gained_magic_talents: Vec<(FullMagicTalent, Vec<ModifierSelections>, String)>,

    // Much like path/magic talents, spells can have mods, and thus can trigger additional lookups, which
    // means we have to load them during the runtime of `collect_from_modifier_tree`. However, because
    // spells are not affected by the LOSE mod, we don't need to store the mod subtree for spells, we
    // can merge them immediately and just return the full spell record.
    pub gained_spells: Vec<(FullSpell, String)>,

    pub required_choices: Vec<FullModifier>,
    pub selected_choices: Vec<(FullModifier, Vec<CharacterEffect>)>,
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
            gained_professions: Vec::new(),
            gained_spells: Vec::new(),
            gained_tradition_ids: Vec::new(),
            modified_slots: Vec::new(),
            statblock_overrides: Vec::new(),
            lost_talents: Vec::new(),
            lost_magic_talents: Vec::new(),
            selected_choices: Vec::new(),
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
        self.gained_professions
            .append(&mut other.gained_professions);
        self.gained_spells.append(&mut other.gained_spells);
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
        self.selected_choices.append(&mut other.selected_choices);
        self.required_choices.append(&mut other.required_choices);
    }

    pub fn collect_losses(&self) -> Losses {
        let mut losses = Losses::new();

        for talent in &self.lost_talents {
            losses.path_talents.insert(talent.clone());
        }
        for (_, sub_trees, _) in &self.gained_talents {
            for sub_tree in sub_trees {
                losses.merge(sub_tree.collect_losses());
            }
        }

        for talent in &self.lost_magic_talents {
            losses.magic_talents.insert(talent.clone());
        }
        for (_, sub_trees, _) in &self.gained_magic_talents {
            for sub_tree in sub_trees {
                losses.merge(sub_tree.collect_losses());
            }
        }

        losses
    }

    pub fn collect(mut self, losses: &Losses) -> Self {
        let path_talents: Vec<(FullPathTalent, Vec<ModifierSelections>, String)> =
            mem::replace(&mut self.gained_talents, Vec::new())
                .into_iter()
                .filter(|(talent, ..)| {
                    !losses
                        .path_talents
                        .contains(&(talent.source.to_owned(), talent.name.to_owned()))
                })
                .map(|(talent, sub_trees, source)| {
                    sub_trees
                        .into_iter()
                        .for_each(|sub_tree| self.merge(sub_tree.collect(&losses)));
                    (talent, Vec::new(), source)
                })
                .collect();
        let magic_talents: Vec<(FullMagicTalent, Vec<ModifierSelections>, String)> =
            mem::replace(&mut self.gained_magic_talents, Vec::new())
                .into_iter()
                .filter(|(talent, ..)| {
                    !losses
                        .magic_talents
                        .contains(&(talent.tradition_name.to_owned(), talent.name.to_owned()))
                })
                .map(|(talent, sub_trees, source)| {
                    sub_trees
                        .into_iter()
                        .for_each(|sub_tree| self.merge(sub_tree.collect(&losses)));
                    (talent, Vec::new(), source)
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

/**
 * Performs the first pass through the modifier tree to collect all losses.
 * (this could also be used to identify a situation where ancestry traits are
 * explicitly ignored, needed for a couple master talents).
 *
 * This pass only performs a minimal search, only nagivating modifiers that are
 * permanent and unconditional, because that's the requirement for LOSE modifiers to
 * work correctly. Because of that, spell mods  and choice selections are ignored,
 * only path talents and magic talents can trigger a LOSE. */
pub fn modifier_tree_first_pass(
    db: Pool<Sqlite>,
    modifier: FullModifier,
) -> Pin<Box<dyn Future<Output = WWResult<Losses>>>> {
    Box::pin(async move {
        let origin_string = modifier.path_node.source_string();
        let mut losses = Losses::new();
        if modifier.mod_details.when.is_permanent()
            && modifier.mod_details.condition.unconditional()
        {
            match &modifier.mod_details.target {
                Target::Lose(lose_target) => match lose_target {
                    LoseTarget::Talent(source, talent) => {
                        losses.path_talents.insert((source.into(), talent.into()));
                    }
                    LoseTarget::MagicTalent(tradition, talent) => {
                        losses
                            .magic_talents
                            .insert((tradition.into(), talent.into()));
                    }
                },
                Target::Grant(grant_target) => match grant_target {
                    GrantTarget::Talent(source, talent) => {
                        let granted_talent = path_talents::get_by_name_and_source(
                            &db,
                            talent,
                            source,
                            &origin_string,
                        )
                        .await?;
                        for talent_mod in granted_talent.modifiers {
                            losses.merge(modifier_tree_first_pass(db.clone(), talent_mod).await?);
                        }
                    }
                    GrantTarget::MagicTalent(tradition, talent) => {
                        let granted_talent =
                            magic_talents::get_by_name_and_tradition(&db, tradition, talent)
                                .await?;
                        for talent_mod in granted_talent.modifiers {
                            losses.merge(modifier_tree_first_pass(db.clone(), talent_mod).await?);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        };

        Ok(losses)
    })
}

/**
 * Performs the second pass through this modifier tree, which is a full search for all effects
 * generated by modifiers, skipping over any talents identified by the first pass.
 *
 * The first version of this process (called collect_from_modifier_tree) was written to be optimal
 * by doing as many tasks in parallel as it could. The issue with that approach is that because
 * modifiers could theoretically be processed in any order, it meant conditions could not be
 * implemented reliably. If a condition relied on whether a trait existed on the player, but that
 * trait was only provided by a separate modifier, then the condition could flop between met and
 * unmet on any given execution.
 * 
 * This version of the modifier tree is now serial, but deterministic. It has the potential to
 * be slow, but my hope is that it's still fast enough that the player would never notice. The
 * biggest factor to time in this function is from DB queries to spells, talents, and choice
 * selection tables. Caching all of those records in advance like we do with the effect cache would
 * be excessive for getting the user, BUT if I really need to improve the speed of this function,
 * it could be solved by implementing a caching layer for the individual queries used here. */
pub fn modifier_tree_second_pass(
    db: Pool<Sqlite>,
    modifier: FullModifier,
    effect_cache: Arc<HashMap<String, CharacterEffect>>,
    // processed_mods: HashSet<FullModifier>,
    processed_effects: DashSet<i64>,
) -> Pin<Box<dyn Future<Output = WWResult<ModifierSelections>>>> {
    Box::pin(async move {
        let origin = modifier.path_node.source_string();
        let mut selections = ModifierSelections::new();
        let is_permanent = modifier.mod_details.when.is_permanent();

        match modifier.mod_details.target.clone() {
            Target::Grant(grant_target) => {
                // Grant modifiers always have exactly 1 key
                let key = modifier.keys().pop().unwrap();
                let (effect_id, cast_hour) = effect_cache
                    .get(&key)
                    .map(|effect| (Some(effect.id), effect.cast_hour))
                    .unwrap_or((None, -1));

                // TODO: Calculate conditions here
                let can_apply = is_permanent || effect_id.is_some();

                if !can_apply {
                    return Ok(selections);
                }

                if let Some(id) = effect_id {
                    // This records all entries from the character_effects table that are applied to the
                    // player. At the end of the process, all rows in the character_effects table with an
                    // id found in this set are preserved, and the rest are deleted.
                    processed_effects.insert(id);
                }

                match grant_target {
                    GrantTarget::Int(x) => {
                        selections.intellect += x as i64;
                    }
                    GrantTarget::Str(x) => {
                        selections.strength += x as i64;
                    }
                    GrantTarget::Will(x) => {
                        selections.will += x as i64;
                    }
                    GrantTarget::Agl(x) => {
                        selections.agility += x as i64;
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
                    GrantTarget::Language(items) => {
                        for name in items {
                            selections.gained_languages.push(GainedLanguage {
                                name,
                                source: origin.clone(),
                                cast_hour,
                            });
                        }
                    }
                    GrantTarget::Immunity(items) => {
                        for name in items {
                            selections.gained_immunities.push(GainedImmunity {
                                name,
                                source: origin.clone(),
                                cast_hour,
                            });
                        }
                    }
                    GrantTarget::Sense(items) => {
                        for (name, amount) in items {
                            selections.gained_senses.push(GainedSense {
                                name,
                                amount,
                                source: origin.clone(),
                                cast_hour,
                            });
                        }
                    }
                    GrantTarget::SpeedTrait(items) => {
                        for (name, amount) in items {
                            selections.gained_speed_traits.push(GainedSpeedTrait {
                                name,
                                amount,
                                source: origin.clone(),
                                cast_hour,
                            });
                        }
                    }
                    GrantTarget::Tradition(items) => {
                        for name in items {
                            selections.gained_traditions.push(GainedTradition {
                                name: name.clone(),
                                source: origin.clone(),
                                cast_hour,
                            });

                            // TODO: A granted tradition triggers a magic talent
                            let constructed_magic_talent_modifier = FullModifier {
                                path_node: modifier.path_node.to_owned(),
                                mod_details: Modifier {
                                    idx: 0,
                                    when: modifier.mod_details.when,
                                    condition: modifier.mod_details.condition.clone(),
                                    target: Target::Choose(ChooseTarget::MagicTalent(
                                        1,
                                        vec![name],
                                    )),
                                },
                            };

                            selections.merge(
                                modifier_tree_second_pass(
                                    db.clone(),
                                    constructed_magic_talent_modifier,
                                    effect_cache.clone(),
                                    processed_effects.clone(),
                                )
                                .await?,
                            );
                        }
                    }
                    GrantTarget::Talent(source, talent) => {
                        let talent =
                            path_talents::get_by_name_and_source(&db, &talent, &source, &origin)
                                .await?;
                        for sub_modifier in talent.modifiers {
                            selections.merge(
                                modifier_tree_second_pass(
                                    db.clone(),
                                    sub_modifier,
                                    effect_cache.clone(),
                                    processed_effects.clone(),
                                )
                                .await?,
                            );
                        }
                    }
                    GrantTarget::MagicTalent(tradition, talent) => {
                        let talent =
                            magic_talents::get_by_name_and_tradition(&db, &tradition, &talent)
                                .await?;
                        for sub_modifier in talent.modifiers {
                            selections.merge(
                                modifier_tree_second_pass(
                                    db.clone(),
                                    sub_modifier,
                                    effect_cache.clone(),
                                    processed_effects.clone(),
                                )
                                .await?,
                            );
                        }
                    }
                }
            }

            Target::Choose(choose_target) => {
                match choose_target {
                    ChooseTarget::Score(_) => {
                        for effect in handle_choice(&modifier, effect_cache, &mut selections)? {
                            parse_score_choice(&effect.selection, &mut selections)?;
                        }
                    }
                    ChooseTarget::Language(_) => {
                        for effect in handle_choice(&modifier, effect_cache, &mut selections)? {
                            selections.gained_language_ids
                                .push(GainedLanguageId {
                                    id: parse_id_choice(&effect.selection)?,
                                    source: origin.clone(),
                                    cast_hour: effect.cast_hour,
                                });
                        }
                    }
                    ChooseTarget::Profession(_) => {
                        for effect in handle_choice(&modifier, effect_cache, &mut selections)? {
                            let profession = professions::get(&db, parse_id_choice(&effect.selection)?).await?;
                            selections.gained_professions.push(GainedProfession {
                                profession,
                                source: origin.clone(),
                                cast_hour: effect.cast_hour,
                            });
                        }
                    }
                    ChooseTarget::NoviceSpell(_)
                    | ChooseTarget::NoviceSpellFrom(_, _)
                    | ChooseTarget::ExpertSpell(_)
                    | ChooseTarget::ExpertSpellFrom(_, _)
                    | ChooseTarget::MasterSpell(_)
                    | ChooseTarget::MasterSpellFrom(_, _) => {
                        for effect in handle_choice(&modifier, effect_cache.clone(), &mut selections)? {
                            let spell_id = parse_id_choice(&effect.selection)?;
                            let spell = spells::get(&db, spell_id).await?;
                            for sub_modifier in spell.modifiers {
                                selections.merge(
                                    modifier_tree_second_pass(
                                        db.clone(),
                                        sub_modifier,
                                        effect_cache.clone(),
                                        processed_effects.clone(),
                                    ).await?,
                                );
                            }
                        }
                    }
                    _ => todo!(),
                };
            }

            // The remaining target types are Apply and Lose
            // Lose modifiers are handled in the first-pass, so ignored in the second pass
            // Apply only contains 1-off modifiers, so nothing is saved
            _ => {}
        };

        Ok(selections)
    })
}

/**
 * This function does several things (which is bad design, so sue me) to process a choice modifier:
 * It iterates through the modifier keys and checks for their values in the effect table (via the
 * cache). If at least one is not provided (in theory, it should be all-or-nothing) and the
 * modifier is marked as required (permanent and condition met), then the modifier is pushed to the
 * `required_choices` list so the player can make decisions. If all effects are found in the effect
 * cache, then the modifier is instead passed into the `selected_choices` list so the player can
 * see and dismiss them. Then the results (each individual effect) is returned from the function so
 * that any additional logic can be run (adding the selected resource to the player, recursing if
 * any talents or spells are gained, etc).
 */
fn handle_choice(
    modifier: &FullModifier,
    effect_cache: Arc<HashMap<String, CharacterEffect>>,
    selections: &mut ModifierSelections,
) -> WWResult<Vec<CharacterEffect>> {
    let mut results = Vec::new();
    let mut choice_required = false;

    for key in modifier.keys() {
        match effect_cache.get(&key) {
            Some(effect) => {
                results.push(effect.clone());
            }
            None => {
                // TODO: Handle conditions here
                if modifier.required() {
                    choice_required = true;
                }
            }
        }
    }

    if choice_required {
        selections.required_choices.push(modifier.clone());
    } else if results.len() > 1 {
        selections
            .selected_choices
            .push((modifier.clone(), results.clone()));
        return Ok(results)
    }

    Ok(Vec::new())

}

fn parse_score_choice(entry: &str, selections: &mut ModifierSelections) -> WWResult<()> {
    match entry {
        "strength" => selections.strength += 1,
        "agility" => selections.agility += 1,
        "intellect" => selections.intellect += 1,
        "will" => selections.will += 1,
        _ => {
            return Err(Generic(format!(
                "Failed to parse Score choice selection '{entry}'"
            )))
        }
    }

    Ok(())
}

fn parse_id_choice(entry: &str) -> WWResult<i64> {
    Ok(entry
        .parse()
        .map_err(|_| Generic(format!("Failed to parse ID value '{entry}'. {DISCLAIMER}")))?)
}

fn parse_tradition_ids_choice(
    entry: &str,
    _: &mut ModifierSelections,
) -> WWResult<(i64, String, i64)> {
    let make_error = || {
        Generic(format!(
            "Failed to parse tradition decision '{entry}'. {DISCLAIMER}"
        ))
    };

    let mut twin_ids = entry.split('|');
    let trad_id = twin_ids
        .next()
        .ok_or_else(make_error)?
        .parse()
        .map_err(|_| Generic(format!("Failed to parse ID value '{entry}'. {DISCLAIMER}")))?;
    let kind = twin_ids.next().ok_or_else(make_error)?.to_string();
    let item_id = twin_ids
        .next()
        .ok_or_else(make_error)?
        .parse()
        .map_err(|_| Generic(format!("Failed to parse ID value '{entry}'. {DISCLAIMER}")))?;

    Ok((trad_id, kind, item_id))
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
