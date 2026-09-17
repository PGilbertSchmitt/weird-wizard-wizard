use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    mod_dsl::{ast::Modifier, parser::parse_mods},
    WWError::Generic,
    WWResult,
};

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "modifiers.ts")]
pub struct FullModifier {
    pub path_str: String,
    pub mod_details: Modifier,
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
                    format!("{};;{}", self.path_str, choice_str),
                )
            })
            .collect()
    }

    pub fn set_choice_strings(&mut self, new_strings: Vec<String>) {
        self.mod_details.target.replace_choice_strings(new_strings);
    }

    pub fn from_path_talent(
        name: &str,
        source: &str,
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
                        path_str: format!("PathTalent;{};{};{}", name, source, idx),
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
                        path_str: format!("MagicTalent;{};{};{}", name, tradition_name, idx),
                    })
                    .collect()
            }
        };

        Ok(modifiers)
    }
}
