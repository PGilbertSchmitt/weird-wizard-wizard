use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    WWError::Generic, WWResult, db::{magic_talents::FullMagicTalent, path_talents::FullPathTalent}, mod_dsl::{ast::{Modifier, WhenMod}, parser::parse_mods},
};

#[derive(TS, Debug, Serialize, Deserialize, Clone)]
#[ts(export, export_to = "modifiers.ts")]
pub struct FullModifier {
    pub path_str: String,
    pub mod_details: Modifier,
}

impl FullModifier {
    pub fn required(&self) -> bool {
        self.mod_details.when.is_permanent()
    }

    pub fn keys(&self) -> Vec<String> {
        self.mod_details
            .target
            .choice_strings()
            .iter()
            .map(|choice_str| format!("{};;{}", self.path_str, choice_str))
            .collect()
    }

    pub fn from_path_talent(talent: &FullPathTalent) -> WWResult<Vec<Self>> {
        let modifiers = match &talent.mod_str {
            None => Vec::new(),
            Some(mod_str) => {
                let raw_modifiers = parse_mods(&mod_str).map_err(|err| Generic(err))?;
                raw_modifiers
                    .into_iter()
                    .enumerate()
                    .map(|(idx, mod_details)| Self {
                        mod_details,
                        path_str: format!("PathTalent;{};{};{}", talent.name, talent.source, idx),
                    })
                    .collect()
            }
        };

        Ok(modifiers)
    }

    pub fn from_magic_talent(talent: &FullMagicTalent) -> WWResult<Vec<Self>> {
        let modifiers = match &talent.mod_str {
            None => Vec::new(),
            Some(mod_str) => {
                let raw_modifiers = parse_mods(&mod_str).map_err(|err| Generic(err))?;
                raw_modifiers
                    .into_iter()
                    .enumerate()
                    .map(|(idx, mod_details)| Self {
                        mod_details,
                        path_str: format!(
                            "MagicTalent;{};{};{}",
                            talent.name, talent.tradition_name, idx
                        ),
                    })
                    .collect()
            }
        };

        Ok(modifiers)
    }
}
