use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts")]
pub struct Modifier {
    pub when: WhenMod,
    pub target: Target,
    pub condition: Condition,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum Target {
    Grant(GrantTarget),
    Lose(LoseTarget),
    Override(OverrideTarget),
    Choose(ChooseTarget, Vec<String>),
    Apply(ApplyTarget),
}

impl Target {
    pub fn choice_strings(&self) -> Vec<String> {
        match self {
            Self::Choose(_, s) => s.clone(),
            _ => Vec::new(),
        }
    }

    pub fn replace_choice_strings(&mut self, new_strings: Vec<String>) {
        match self {
            Self::Choose(_, old_strings) => {
                *old_strings = new_strings;
            }
            _ => {}
        }
    }
}

#[derive(TS, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum WhenDuration {
    OneMinute,
    OneHour,
    FourHours,
    EightHours,
    OneDay,
    Rest,
}

impl WhenDuration {
    pub fn to_string(&self) -> String {
        match self {
            Self::OneMinute => String::from("OneMinute"),
            Self::OneHour => String::from("OneHour"),
            Self::FourHours => String::from("FourHours"),
            Self::EightHours => String::from("EightHours"),
            Self::OneDay => String::from("OneDay"),
            Self::Rest => String::from("Rest"),
        }
    }
}

#[derive(TS, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum WhenMod {
    Permanent,
    CastOnce,
    CastTime(WhenDuration),
    CastTimeDismiss(WhenDuration),
}

impl WhenMod {
    pub fn is_permanent(&self) -> bool {
        match self {
            Self::Permanent => true,
            _ => false,
        }
    }

    /* Returns (dismissable, Option<duration>) */
    pub fn to_choice_data(&self) -> (bool, Option<String>) {
        match self {
            Self::CastOnce => (true, None),
            Self::Permanent => (false, None),
            Self::CastTime(when) => (false, Some(when.to_string())),
            Self::CastTimeDismiss(when) => (true, Some(when.to_string())),
        }
    }
}

// Grant - Everything except StatBlock, Heal, and Slots
#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum GrantTarget {
    Int(i32),
    Str(i32),
    Will(i32),
    Agl(i32),
    Speed(i32),
    Health(i32),
    Defense(i32),
    NatDef(i32),
    BonusDamage(i32),
    Language(Vec<String>),
    Tradition(Vec<String>),
    Immunity(Vec<String>),
    // Sense and Speed Trait both have optional quantities
    Sense(Vec<(String, Option<String>)>),
    SpeedTrait(Vec<(String, Option<String>)>),
    Talent(String, String),
    MagicTalent(String, String),
}

// Lose - Only Talent and MagicTalent
#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum LoseTarget {
    Talent(String, String),
    MagicTalent(String, String),
}

// Override - Only Speed, Defense, the unique StatBlock target, and the unique MergeStatBlock target
#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum OverrideTarget {
    Speed(i32),
    Defense(i32),
    StatBlock(String),
    MergeStatBlock(String, String),
}

// Choose - This MOD uses its own targets, which don't overlap with any of the others
#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ChooseTarget {
    Language(u32),
    Profession(u32),
    Tradition(u32),
    NoviceSpell(u32),
    NoviceSpellFrom(u32, Vec<String>),
    ExpertSpell(u32),
    ExpertSpellFrom(u32, Vec<String>),
    MasterSpell(u32),
    MasterSpellFrom(u32, Vec<String>),
    Select(u32, String),
    Score(u32),
    Slots(ChooseSlotTarget),
    MagicTalent(u32, Vec<String>),
}

impl ChooseTarget {
    pub fn choice_strings(&self) -> Vec<String> {
        match self {
            Self::Language(count) => repeat_id("LANGUAGE", *count),
            Self::Profession(count) => repeat_id("PROFESSION", *count),
            Self::Tradition(count) => repeat_id("TRADITION", *count),
            Self::NoviceSpell(count) => repeat_id("NOVICE_SPELL", *count),
            Self::NoviceSpellFrom(count, trads) => {
                repeat_id(&format!("NOVICE_SPELL({})", trads.join(",")), *count)
            }
            Self::ExpertSpell(count) => repeat_id("EXPERT_SPELL", *count),
            Self::ExpertSpellFrom(count, trads) => {
                repeat_id(&format!("EXPERT_SPELL({})", trads.join(",")), *count)
            }
            Self::MasterSpell(count) => repeat_id("MASTER_SPELL", *count),
            Self::MasterSpellFrom(count, trads) => {
                repeat_id(&format!("MASTER_SPELL({})", trads.join(",")), *count)
            }
            Self::Select(count, choice_id) => repeat_id(&format!("SELECT='{choice_id}'"), *count),
            Self::Score(count) => repeat_id("SCORE", *count),
            Self::Slots(target) => vec![format!("SLOTS='{target}'")],
            Self::MagicTalent(count, trads) => {
                repeat_id(&format!("MagicTalent({})", trads.join(",")), *count)
            }
        }
    }
}

fn repeat_id(s: &str, count: u32) -> Vec<String> {
    (0..count).map(|x| format!("[{x}]{s}")).collect()
}

// Apply - Only Heal, Health, and Slots
#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ApplyTarget {
    Heal(HealAmount),
    Health(HealAmount),
    Slots(ApplySlotsTarget),
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum HealAmount {
    Number(i32),
    Expr(SimpleExpr),
    Roll(Dice),
    Ask(String),
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ChooseSlotTarget {
    Plus(u32),
    Times(u32),
}

impl Display for ChooseSlotTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Plus(x) => {
                f.write_char('+')?;
                f.write_str(&x.to_string())
            }
            Self::Times(x) => {
                f.write_char('*')?;
                f.write_str(&x.to_string())
            }
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ApplySlotsTarget {
    AnySpell(u32),
    NoviceSpell(u32),
    ExpertSpell(u32),
    MasterSpell(u32),
    Tradition(u32, String),
    SpecificSpell(u32, String, String),
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts")]
pub struct Dice {
    pub die_size: i32,
    pub die_count: i32,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum HasCategory {
    SpeedTrait(String),
    Sense(String),
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts")]
pub struct SimpleExpr {
    pub left: ExprValue,
    pub right: Option<(ExprOp, ExprValue)>,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ExprValue {
    Health,
    Dmg,
    Level,
    Defense,
    Number(i32),
}

impl Display for ExprValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Health => f.write_str("HEALTH"),
            Self::Dmg => f.write_str("DMG"),
            Self::Level => f.write_str("LEVEL"),
            Self::Defense => f.write_str("DEF"),
            Self::Number(x) => f.write_str(&x.to_string()),
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum ExprOp {
    Lt,
    Gt,
    LtEq,
    GtEq,
    Eq,
    Div,
}

impl Display for ExprOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Lt => f.write_char('<'),
            Self::Gt => f.write_char('>'),
            Self::LtEq => f.write_str("<="),
            Self::GtEq => f.write_str(">="),
            Self::Eq => f.write_char('='),
            Self::Div => f.write_char('/'),
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum Condition {
    None,
    HasNot,
    Has(HasCategory),
    If(SimpleExpr),
}
