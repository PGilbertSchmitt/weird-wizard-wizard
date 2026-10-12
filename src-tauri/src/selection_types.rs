use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::db::{
    languages::Language, magic_talents::FullMagicTalent, path_talents::FullPathTalent,
    professions::Profession, senses::FullSense, speed_traits::FullSpeedTrait, spells::FullSpell,
    traditions::PartialTradition,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedLanguage {
    pub language: Language,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedImmunity {
    pub name: String, // A String is all we need here
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedTradition {
    pub tradition: PartialTradition,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedSense {
    pub sense: FullSense,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedSpeedTrait {
    pub speed_trait: FullSpeedTrait,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedProfession {
    pub profession: Profession,
    pub source: String,
    // I don't know why anyone would create mods that add professions temporarily, but It's trivial
    // to support, so it's no beans off my bone.
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedPathTalent {
    pub talent: FullPathTalent,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedMagicTalent {
    pub talent: FullMagicTalent,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "character.ts")]
pub struct GainedSpell {
    pub spell: FullSpell,
    pub source: String,
    pub cast_hour: i64,
}
