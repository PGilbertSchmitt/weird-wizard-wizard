use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, sqlx::Type)]
#[ts(export, export_to = "etc.ts")]
#[sqlx(type_name = "TEXT")]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl From<String> for Size {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "sm" => Self::Sm,
            "md" => Self::Md,
            "lg" => Self::Lg,
            _ => Self::Md,
        }
    }
}

#[derive(TS, Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[ts(export, export_to = "etc.ts")]
#[sqlx(type_name = "TEXT")]
pub enum PathKind {
    Novice,
    Expert,
    Master,
}

impl From<String> for PathKind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "novice" => Self::Novice,
            "expert" => Self::Expert,
            "master" => Self::Master,
            _ => Self::Novice,
        }
    }
}

#[derive(TS, Debug, Serialize, Deserialize, sqlx::Type)]
#[ts(export, export_to = "etc.ts")]
#[sqlx(type_name = "TEXT")]
pub enum TalentRestore {
    None,
    LuckEnds,
    Rest,
    Day,
    Hour,
    Minute,
    StartOfNextTurn,
    EndOfNextTurn,
    StartOfRound,
    Special,
}

impl From<String> for TalentRestore {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "None" => Self::None,
            "Luck Ends" => Self::LuckEnds,
            "Rest" => Self::Rest,
            "Day" => Self::Day,
            "Hour" => Self::Hour,
            "Minute" => Self::Minute,
            "Start of Next Turn" => Self::StartOfNextTurn,
            "End of Next Turn" => Self::EndOfNextTurn,
            "Start of Round" => Self::StartOfRound,
            "Special" => Self::Special,
            _ => Self::None,
        }
    }
}
