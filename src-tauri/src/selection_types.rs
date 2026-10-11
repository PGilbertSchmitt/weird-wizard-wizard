use crate::db::professions::Profession;

#[derive(Debug)]
pub struct GainedLanguage {
    pub name: String,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(Debug)]
pub struct GainedImmunity {
    pub name: String,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(Debug)]
pub struct GainedTradition {
    pub name: String,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(Debug)]
pub struct GainedSense {
    pub name: String,
    pub amount: Option<String>,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(Debug)]
pub struct GainedSpeedTrait {
    pub name: String,
    pub amount: Option<String>,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(Debug)]
pub struct GainedLanguageId {
    pub id: i64,
    pub source: String,
    pub cast_hour: i64,
}

#[derive(Debug)]
pub struct GainedProfession {
    pub profession: Profession,
    pub source: String,
    // I don't know why anyone would create mods that add professions temporarily, but It's trivial
    // to support, so it's no beans off my bone.
    pub cast_hour: i64,
}


// GainedSenses: Vec<(String, Option<String>, String)>
// GainedSpeedTraits: Vec<(String, Option<String>, String)>
// GainedTraditionIds: Vec<(i64, String)>
// GainedProfessionIds: Vec<i64>
// GainedTalents: Vec<(FullPathTalent, Vec<ModifierSelections>, String)>
// GainedMagicTalents: Vec<(FullMagicTalent, Vec<ModifierSelections>, String)>
// GainedSpells: Vec<(FullSpell, String)>
