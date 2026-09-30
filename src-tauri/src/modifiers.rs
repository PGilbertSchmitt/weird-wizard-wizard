use std::fmt::Display;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    mod_dsl::{ast::Modifier, parser::parse_mods},
    WWError::Generic,
    WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts")]
pub struct FullModifier {
    pub path_node: ModifierPathNode,
    pub mod_details: Modifier,
}

// This identifies a single leg of a modifier path. There's one kind per source of a Modifier.
// The idx is needed because a single mod_str can have multiple Modifiers.
#[derive(TS, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ModifierPathNode {
    LevelLanguage {
        path_name: String,
        level: i64,
    },
    LevelTradition {
        path_name: String,
        level: i64,
    },
    LevelMagicTalent {
        path_name: String,
        level: i64,
    },
    LevelNoviceSpell {
        path_name: String,
        level: i64,
    },
    LevelExpertSpell {
        path_name: String,
        level: i64,
    },
    LevelMasterSpell {
        path_name: String,
        level: i64,
    },
    LevelScore {
        level: i64,
    },
    PathTalent {
        name: String,
        source: String,
        granter: String,
        idx: usize,
    },
    MagicTalent {
        name: String,
        tradition: String,
        idx: usize,
    },
    Spell {
        name: String,
        tradition: String,
        idx: usize,
    },
    ChoiceSelection {
        name: String,
        label: String,
        idx: usize,
    },
}

// Used to create keys into the character_choices table
impl Display for ModifierPathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LevelLanguage { path_name, level } => {
                f.write_fmt(format_args!("level_language|{path_name}|{level}"))?;
            }
            Self::LevelTradition { path_name, level } => {
                f.write_fmt(format_args!("level_tradition|{path_name}|{level}"))?;
            }
            Self::LevelMagicTalent { path_name, level } => {
                f.write_fmt(format_args!("level_tradition_talent|{path_name}|{level}"))?;
            }
            Self::LevelNoviceSpell { path_name, level } => {
                f.write_fmt(format_args!("level_novice_spell|{path_name}|{level}"))?;
            }
            Self::LevelExpertSpell { path_name, level } => {
                f.write_fmt(format_args!("level_expert_spell|{path_name}|{level}"))?;
            }
            Self::LevelMasterSpell { path_name, level } => {
                f.write_fmt(format_args!("level_master_spell|{path_name}|{level}"))?;
            }
            Self::LevelScore { level } => {
                f.write_fmt(format_args!("score_gain|{level}"))?;
            }
            Self::PathTalent { name, source, granter, idx } => {
                f.write_fmt(format_args!("path_talent|{name}|{source}|{granter}|{idx}"))?;
            }
            Self::MagicTalent {
                name,
                tradition,
                idx,
            } => {
                f.write_fmt(format_args!("magic_talent|{name}|{tradition}|{idx}"))?;
            }
            Self::Spell {
                name,
                tradition,
                idx,
            } => {
                f.write_fmt(format_args!("spell|{name}|{tradition}|{idx}"))?;
            }
            Self::ChoiceSelection { name, label, idx } => {
                f.write_fmt(format_args!("choice_selection|{name}|{label}|{idx}"))?;
            }
        };
        Ok(())
    }
}

impl ModifierPathNode {
    pub fn source_string(&self) -> String {
        match self {
            Self::LevelLanguage { path_name, level }
            | Self::LevelTradition { path_name, level }
            | Self::LevelMagicTalent { path_name, level }
            | Self::LevelNoviceSpell { path_name, level }
            | Self::LevelExpertSpell { path_name, level }
            | Self::LevelMasterSpell { path_name, level } => format!("{path_name} level {level}"),
            Self::LevelScore { level } => format!("Level {level} score gain"),
            Self::PathTalent { name, source, .. } => format!("{source} talent {name}"),
            Self::MagicTalent {
                name, tradition, ..
            } => format!("{tradition} talent {name}"),
            Self::Spell {
                name, tradition, ..
            } => format!("{tradition} spell {name}"),
            Self::ChoiceSelection { name, label, .. } => format!("{label} choice {name}"),
        }
    }
}

pub trait HasModifiers {
    fn modifiers(&self) -> Vec<FullModifier>;
}

impl FullModifier {
    pub fn required(&self) -> bool {
        self.mod_details.when.is_permanent()
    }

    pub fn keys(&self) -> Vec<(String, String)> {
        self.mod_details
            .target
            .choice_strings()
            .iter()
            .map(|choice_str| {
                (
                    choice_str.clone(),
                    format!("{}=>{}", self.path_node, choice_str),
                )
            })
            .collect()
    }

    pub fn set_choice_strings(&mut self, new_strings: Vec<String>) {
        self.mod_details.target.replace_choice_strings(new_strings);
    }

    pub fn from_path_talent(
        name: &str,
        // `source` here refers to the `source` column on the paths table
        source: &str,
        // `source` here refers to the more specific source of the talent, which
        // is either an ancestry, a path's specific level, or a GRANT modifier.
        source_string: String,
        mod_str: &Option<String>,
    ) -> WWResult<Vec<Self>> {
        let modifiers = match mod_str {
            None => Vec::new(),
            Some(mod_str) => {
                let raw_modifiers = parse_mods(mod_str).map_err(|err| Generic(err))?;
                raw_modifiers
                    .into_iter()
                    .enumerate()
                    .map(|(idx, mod_details)| Self {
                        mod_details,
                        path_node: ModifierPathNode::PathTalent {
                            name: name.to_string(),
                            source: source.to_string(),
                            granter: source_string.clone(),
                            idx,
                        },
                    })
                    .collect()
            }
        };

        Ok(modifiers)
    }

    pub fn from_magic_talent(
        name: &str,
        tradition_name: &str,
        mod_str: &Option<String>,
    ) -> WWResult<Vec<Self>> {
        let modifiers = match mod_str {
            None => Vec::new(),
            Some(mod_str) => {
                let raw_modifiers = parse_mods(mod_str).map_err(|err| Generic(err))?;
                raw_modifiers
                    .into_iter()
                    .enumerate()
                    .map(|(idx, mod_details)| Self {
                        mod_details,
                        path_node: ModifierPathNode::MagicTalent {
                            name: name.to_string(),
                            tradition: tradition_name.to_string(),
                            idx,
                        },
                    })
                    .collect()
            }
        };

        Ok(modifiers)
    }

    pub fn from_spell(
        name: &str,
        tradition_name: &str,
        mod_str: &Option<String>,
    ) -> WWResult<Vec<Self>> {
        let modifiers = match mod_str {
            None => Vec::new(),
            Some(mod_str) => {
                let raw_modifiers = parse_mods(mod_str).map_err(|err| Generic(err))?;
                raw_modifiers
                    .into_iter()
                    .enumerate()
                    .map(|(idx, mod_details)| Self {
                        mod_details,
                        path_node: ModifierPathNode::Spell {
                            name: name.to_string(),
                            tradition: tradition_name.to_string(),
                            idx,
                        },
                    })
                    .collect()
            }
        };

        Ok(modifiers)
    }
}
