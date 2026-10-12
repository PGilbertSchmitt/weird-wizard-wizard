use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    sync::Arc,
};

use dashmap::DashSet;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use ts_rs::TS;

use crate::{
    db::{
        character_effects::CharacterEffect,
        choice_selections,
        languages::Language,
        magic_talents::{self, FullMagicTalent},
        path_talents, professions,
        senses::{FullSense, RawSense},
        speed_traits::{FullSpeedTrait, RawSpeedTrait},
        spells::{self, FullSpell},
        traditions::{self, PartialTradition},
    },
    mod_dsl::ast::{ChooseTarget, GrantTarget, LoseTarget, Modifier, Target},
    modifiers::{FullModifier, HasModifiers},
    selection_types::{
        GainedImmunity, GainedLanguage, GainedMagicTalent, GainedPathTalent, GainedProfession,
        GainedSense, GainedSpeedTrait, GainedSpell, GainedTradition,
    },
    WWError::Generic,
    WWResult,
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
    pub gained_professions: Vec<GainedProfession>,
    pub statblock_overrides: Vec<(i64, String)>,
    pub modified_slots: Vec<SlotMod>,

    // This is how the ModifierSelections recurses into a tree structure. It will later be collapsed into
    // a flat structure, during which time the accumulated "lost" talents can prune the tree.
    pub gained_path_talents: Vec<GainedPathTalent>,
    pub gained_magic_talents: Vec<GainedMagicTalent>,
    pub gained_spells: Vec<GainedSpell>,

    // Much like path/magic talents, spells can have mods, and thus can trigger additional lookups, which
    // means we have to load them during the runtime of `collect_from_modifier_tree`. However, because
    // spells are not affected by the LOSE mod, we don't need to store the mod subtree for spells, we
    // can merge them immediately and just return the full spell record.
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
            gained_professions: Vec::new(),
            gained_languages: Vec::new(),
            gained_traditions: Vec::new(),
            gained_senses: Vec::new(),
            gained_immunities: Vec::new(),
            gained_speed_traits: Vec::new(),
            gained_path_talents: Vec::new(),
            gained_magic_talents: Vec::new(),
            gained_spells: Vec::new(),
            modified_slots: Vec::new(),
            selected_choices: Vec::new(),
            required_choices: Vec::new(),
            statblock_overrides: Vec::new(),
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
        self.gained_path_talents
            .append(&mut other.gained_path_talents);
        self.gained_professions
            .append(&mut other.gained_professions);
        self.gained_spells.append(&mut other.gained_spells);
        self.modified_slots.append(&mut other.modified_slots);
        self.gained_magic_talents
            .append(&mut other.gained_magic_talents);
        self.statblock_overrides
            .append(&mut other.statblock_overrides);
        self.selected_choices.append(&mut other.selected_choices);
        self.required_choices.append(&mut other.required_choices);
    }
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub enum SlotMod {
    Plus { amount: i64, spell_id: i64 },
    Times { amount: i64, spell_id: i64 },
}

pub struct Losses {
    pub path_talents: HashSet<(String, String)>,
    pub magic_talents: HashSet<(String, String)>,
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
) -> Pin<Box<dyn Future<Output = WWResult<Losses>> + Send>> {
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
 * it could be solved by implementing a caching layer for the individual queries used here.
 *
 * TODO: Implement ordered recursion. This function is deterministic, but the order in which it
 * recurses is not ideal. Rather than recursing directly, calls to itself should be replaced with
 * work items pushed into a priority queue. The priority of items should be determined by origin.
 *
 * TODO: Add Losses from the first pass here so path talents / magic talents can be culled. */
pub fn modifier_tree_second_pass(
    db: Pool<Sqlite>,
    modifier: FullModifier,
    effect_cache: Arc<HashMap<String, CharacterEffect>>,
    language_cache: Arc<Vec<Language>>,
    speed_trait_cache: Arc<Vec<RawSpeedTrait>>,
    sense_cache: Arc<Vec<RawSense>>,
    processed_effects: DashSet<i64>,
) -> Pin<Box<dyn Future<Output = WWResult<ModifierSelections>> + Send>> {
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
                            if let Some(language) =
                                language_cache.iter().find(|lang| lang.name == name)
                            {
                                selections.gained_languages.push(GainedLanguage {
                                    language: language.clone(),
                                    source: origin.clone(),
                                    cast_hour,
                                });
                            } else {
                                return Err(Generic(format!(
                                    "Cannot find language with name {name}"
                                )));
                            };
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
                            if let Some(sense) = sense_cache.iter().find(|sense| sense.name == name)
                            {
                                selections.gained_senses.push(GainedSense {
                                    sense: FullSense {
                                        id: sense.id,
                                        name,
                                        description: sense.description.clone(),
                                        unit: sense.unit.clone(),
                                        amount,
                                    },
                                    source: origin.clone(),
                                    cast_hour,
                                });
                            } else {
                                return Err(Generic(format!("Cannot find sense with name {name}")));
                            }
                        }
                    }
                    GrantTarget::SpeedTrait(items) => {
                        for (name, amount) in items {
                            if let Some(raw_trait) =
                                speed_trait_cache.iter().find(|t| t.name == name)
                            {
                                selections.gained_speed_traits.push(GainedSpeedTrait {
                                    speed_trait: FullSpeedTrait {
                                        id: raw_trait.id,
                                        name,
                                        description: raw_trait.description.clone(),
                                        unit: raw_trait.unit.clone(),
                                        amount,
                                    },
                                    source: origin.clone(),
                                    cast_hour,
                                });
                            } else {
                            }
                        }
                    }
                    GrantTarget::Tradition(items) => {
                        for name in items {
                            let tradition = traditions::get_partial_by_name(&db, &name).await?;
                            selections.gained_traditions.push(GainedTradition {
                                tradition,
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
                                    language_cache.clone(),
                                    speed_trait_cache.clone(),
                                    sense_cache.clone(),
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
                        let talent_modifiers = talent.modifiers();

                        selections.gained_path_talents.push(GainedPathTalent {
                            talent,
                            source: origin.clone(),
                            cast_hour,
                        });

                        for sub_modifier in talent_modifiers {
                            selections.merge(
                                modifier_tree_second_pass(
                                    db.clone(),
                                    sub_modifier,
                                    effect_cache.clone(),
                                    language_cache.clone(),
                                    speed_trait_cache.clone(),
                                    sense_cache.clone(),
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

                        let talent_modifiers = talent.modifiers();

                        selections.gained_magic_talents.push(GainedMagicTalent {
                            talent,
                            source: origin.clone(),
                            cast_hour,
                        });

                        for sub_modifier in talent_modifiers {
                            selections.merge(
                                modifier_tree_second_pass(
                                    db.clone(),
                                    sub_modifier,
                                    effect_cache.clone(),
                                    language_cache.clone(),
                                    speed_trait_cache.clone(),
                                    sense_cache.clone(),
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
                            let lang_id = parse_id_choice(&effect.selection)?;
                            if let Some(language) =
                                language_cache.iter().find(|lang| lang.id == lang_id)
                            {
                                selections.gained_languages.push(GainedLanguage {
                                    language: language.clone(),
                                    source: origin.clone(),
                                    cast_hour: effect.cast_hour,
                                });
                            } else {
                                return Err(Generic(format!(
                                    "Cannot find language with id {lang_id}"
                                )));
                            };
                        }
                    }
                    ChooseTarget::Profession(_) => {
                        for effect in handle_choice(&modifier, effect_cache, &mut selections)? {
                            let profession =
                                professions::get(&db, parse_id_choice(&effect.selection)?).await?;
                            selections.gained_professions.push(GainedProfession {
                                profession,
                                source: origin.clone(),
                                cast_hour: effect.cast_hour,
                            });
                        }
                    }
                    ChooseTarget::Tradition(_) => {
                        for effect in
                            handle_choice(&modifier, effect_cache.clone(), &mut selections)?
                        {
                            let (tradition, talent, spell) =
                                get_tradition_elements(db.clone(), &effect.selection).await?;

                            selections.gained_traditions.push(GainedTradition {
                                tradition,
                                source: origin.clone(),
                                cast_hour: effect.cast_hour,
                            });

                            if let Some(talent) = talent {
                                for sub_modifier in talent.modifiers.clone() {
                                    selections.merge(
                                        modifier_tree_second_pass(
                                            db.clone(),
                                            sub_modifier,
                                            effect_cache.clone(),
                                            language_cache.clone(),
                                            speed_trait_cache.clone(),
                                            sense_cache.clone(),
                                            processed_effects.clone(),
                                        )
                                        .await?,
                                    );
                                }

                                selections.gained_magic_talents.push(GainedMagicTalent {
                                    talent,
                                    source: origin.clone(),
                                    cast_hour: effect.cast_hour,
                                });
                            }

                            if let Some(spell) = spell {
                                for sub_modifier in spell.modifiers.clone() {
                                    selections.merge(
                                        modifier_tree_second_pass(
                                            db.clone(),
                                            sub_modifier,
                                            effect_cache.clone(),
                                            language_cache.clone(),
                                            speed_trait_cache.clone(),
                                            sense_cache.clone(),
                                            processed_effects.clone(),
                                        )
                                        .await?,
                                    );
                                }

                                selections.gained_spells.push(GainedSpell {
                                    spell,
                                    source: origin.clone(),
                                    cast_hour: effect.cast_hour,
                                });
                            }
                        }
                    }
                    ChooseTarget::MagicTalent(..) => {
                        for effect in
                            handle_choice(&modifier, effect_cache.clone(), &mut selections)?
                        {
                            let (_, talent, spell) =
                                get_tradition_elements(db.clone(), &effect.selection).await?;

                            if let Some(talent) = talent {
                                for sub_modifier in talent.modifiers.clone() {
                                    selections.merge(
                                        modifier_tree_second_pass(
                                            db.clone(),
                                            sub_modifier,
                                            effect_cache.clone(),
                                            language_cache.clone(),
                                            speed_trait_cache.clone(),
                                            sense_cache.clone(),
                                            processed_effects.clone(),
                                        )
                                        .await?,
                                    );
                                }

                                selections.gained_magic_talents.push(GainedMagicTalent {
                                    talent,
                                    source: origin.clone(),
                                    cast_hour: effect.cast_hour,
                                });
                            }

                            if let Some(spell) = spell {
                                for sub_modifier in spell.modifiers.clone() {
                                    selections.merge(
                                        modifier_tree_second_pass(
                                            db.clone(),
                                            sub_modifier,
                                            effect_cache.clone(),
                                            language_cache.clone(),
                                            speed_trait_cache.clone(),
                                            sense_cache.clone(),
                                            processed_effects.clone(),
                                        )
                                        .await?,
                                    );
                                }

                                selections.gained_spells.push(GainedSpell {
                                    spell,
                                    source: origin.clone(),
                                    cast_hour: effect.cast_hour,
                                });
                            }
                        }
                    }
                    ChooseTarget::NoviceSpell(_)
                    | ChooseTarget::NoviceSpellFrom(..)
                    | ChooseTarget::ExpertSpell(_)
                    | ChooseTarget::ExpertSpellFrom(..)
                    | ChooseTarget::MasterSpell(_)
                    | ChooseTarget::MasterSpellFrom(..) => {
                        for effect in
                            handle_choice(&modifier, effect_cache.clone(), &mut selections)?
                        {
                            let spell_id = parse_id_choice(&effect.selection)?;
                            let spell = spells::get(&db, spell_id).await?;
                            for sub_modifier in spell.modifiers {
                                selections.merge(
                                    modifier_tree_second_pass(
                                        db.clone(),
                                        sub_modifier,
                                        effect_cache.clone(),
                                        language_cache.clone(),
                                        speed_trait_cache.clone(),
                                        sense_cache.clone(),
                                        processed_effects.clone(),
                                    )
                                    .await?,
                                );
                            }
                        }
                    }
                    ChooseTarget::Select(..) => {
                        for effect in
                            handle_choice(&modifier, effect_cache.clone(), &mut selections)?
                        {
                            let choice_selection = choice_selections::get_full_choice_selection(
                                &db,
                                parse_id_choice(&effect.selection)?,
                            )
                            .await?;
                            for sub_modifier in choice_selection.modifiers() {
                                selections.merge(
                                    modifier_tree_second_pass(
                                        db.clone(),
                                        sub_modifier,
                                        effect_cache.clone(),
                                        language_cache.clone(),
                                        speed_trait_cache.clone(),
                                        sense_cache.clone(),
                                        processed_effects.clone(),
                                    )
                                    .await?,
                                );
                            }
                        }
                    }
                    ChooseTarget::Slots(_) => {
                        for effect in
                            handle_choice(&modifier, effect_cache.clone(), &mut selections)?
                        {
                            let slot_effect = parse_slots_choice(&effect.selection)?;
                            selections.modified_slots.push(slot_effect);
                        }
                    }
                };
            }

            // TODO:
            // Target::Override(override_target) => {}

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

    // TODO: Handle conditions here. Only permanent choices are required, and only choices with
    // conditions that are met can propagate effects.

    for key in modifier.keys() {
        match effect_cache.get(&key) {
            Some(effect) => {
                results.push(effect.clone());
            }
            None => {
                if modifier.required() {
                    choice_required = true;
                }
            }
        }
    }

    if choice_required {
        selections.required_choices.push(modifier.clone());
    } else if results.len() > 0 {
        selections
            .selected_choices
            .push((modifier.clone(), results.clone()));
        return Ok(results);
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

fn parse_tradition_ids_choice(entry: &str) -> WWResult<(i64, String, i64)> {
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

fn parse_slots_choice(entry: &str) -> WWResult<SlotMod> {
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

async fn get_tradition_elements(
    db: Pool<Sqlite>,
    selection: &str,
) -> WWResult<(PartialTradition, Option<FullMagicTalent>, Option<FullSpell>)> {
    let (trad_id, kind, item_id) = parse_tradition_ids_choice(selection)?;
    let db_1 = db.clone();
    let db_2 = db.clone();
    let is_talent = kind == "t";
    let (tradition, talent, spell) = futures::join!(
        traditions::get_partial(&db, trad_id),
        async move {
            if is_talent {
                Some(magic_talents::get(&db_1, item_id).await)
            } else {
                None
            }
        },
        async move {
            if !is_talent {
                Some(spells::get(&db_2, item_id).await)
            } else {
                None
            }
        },
    );

    Ok((
        tradition?,
        match talent {
            None => None,
            Some(talent) => Some(talent?),
        },
        match spell {
            None => None,
            Some(spell) => Some(spell?),
        },
    ))
}
