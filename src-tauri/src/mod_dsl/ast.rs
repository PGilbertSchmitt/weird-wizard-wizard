use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult, Write},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts")]
pub struct Modifier {
    pub idx: usize, // position within a mod str
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
    Choose(ChooseTarget),
    Apply(ApplyTarget),
}

impl Target {
    // Choose targets can generate multiple keys, Apply never generates keys, and the
    // the rest always generate exactly 1 key.
    pub fn target_keys(&self) -> Vec<String> {
        match self {
            Self::Choose(target) => target.keys(),
            Self::Grant(target) => target.keys(),
            Self::Lose(target) => target.keys(),
            Self::Override(target) => target.keys(),
            // Apply is explicitly a 1-time effect, so no keys are needed to record it
            Self::Apply(_) => Vec::new(),
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

    pub fn is_rest(&self) -> bool {
        match self {
            Self::Rest => true,
            _ => false,
        }
    }

    /**
     * When a duration is passed, this function returns the list of all strings derived from
     * WhenDuration values which are contained withiin it. The values here are specific decisions.
     */
    pub fn contained_duration_strings(&self) -> Vec<String> {
        match self {
            Self::OneMinute => vec![Self::OneMinute.to_string()],
            Self::OneHour => vec![Self::OneHour.to_string(), Self::OneMinute.to_string()],
            Self::FourHours => vec![
                Self::FourHours.to_string(),
                Self::OneHour.to_string(),
                Self::OneMinute.to_string(),
            ],
            // A rest is 8 hours, but just because that amount of time passes doesn't mean a rest has occured
            Self::EightHours => vec![
                Self::EightHours.to_string(),
                Self::FourHours.to_string(),
                Self::OneHour.to_string(),
                Self::OneMinute.to_string(),
            ],
            Self::OneDay => vec![
                Self::OneDay.to_string(),
                Self::EightHours.to_string(),
                Self::FourHours.to_string(),
                Self::OneHour.to_string(),
                Self::OneMinute.to_string(),
            ],
            Self::Rest => vec![
                Self::EightHours.to_string(),
                Self::FourHours.to_string(),
                Self::OneHour.to_string(),
                Self::OneMinute.to_string(),
            ],
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

impl GrantTarget {
    pub fn keys(&self) -> Vec<String> {
        match self {
            Self::Int(_) => vec!["GRANT_INT".into()],
            Self::Str(_) => vec!["GRANT_STR".into()],
            Self::Will(_) => vec!["GRANT_WILL".into()],
            Self::Agl(_) => vec!["GRANT_AGL".into()],
            Self::Speed(_) => vec!["GRANT_SPEED".into()],
            Self::Health(_) => vec!["GRANT_HEALTH".into()],
            Self::Defense(_) => vec!["GRANT_DEFENSE".into()],
            Self::NatDef(_) => vec!["GRANT_NATDEF".into()],
            Self::BonusDamage(_) => vec!["GRANT_BONUSDAMAGE".into()],
            Self::Language(_) => vec!["GRANT_LANGUAGE".into()],
            Self::Tradition(_) => vec!["GRANT_TRADITION".into()],
            Self::Immunity(_) => vec!["GRANT_IMMUNITY".into()],
            Self::Sense(_) => vec!["GRANT_SENSE".into()],
            Self::SpeedTrait(_) => vec!["GRANT_SPEEDTRAIT".into()],
            Self::Talent(..) => vec!["GRANT_TALENT".into()],
            Self::MagicTalent(..) => vec!["GRANT_MAGICTALENT".into()],
        }
    }
}

// Lose - Only Talent and MagicTalent
#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum LoseTarget {
    Talent(String, String),
    MagicTalent(String, String),
}

impl LoseTarget {
    pub fn keys(&self) -> Vec<String> {
        match self {
            Self::Talent(..) => vec!["LOSE_TALENT".into()],
            Self::MagicTalent(..) => vec!["LOSE_MAGICTALENT".into()],
        }
    }
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

impl OverrideTarget {
    pub fn keys(&self) -> Vec<String> {
        match self {
            Self::Speed(_) => vec!["OVERRIDE_SPEED".into()],
            Self::Defense(_) => vec!["OVERRIDE_DEFENSE".into()],
            Self::StatBlock(_) => vec!["OVERRIDE_STATBLOCK".into()],
            Self::MergeStatBlock(..) => vec!["OVERRIDE_MERGESTATBLOCK".into()],
        }
    }
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
    pub fn keys(&self) -> Vec<String> {
        match self {
            Self::Language(count) => repeat_id("CHOOSE_LANGUAGE", *count),
            Self::Profession(count) => repeat_id("CHOOSE_PROFESSION", *count),
            Self::Tradition(count) => repeat_id("CHOOSE_TRADITION", *count),
            Self::NoviceSpell(count) => repeat_id("CHOOSE_NOVICE_SPELL", *count),
            Self::NoviceSpellFrom(count, trads) => {
                repeat_id(&format!("CHOOSE_NOVICE_SPELL({})", trads.join(",")), *count)
            }
            Self::ExpertSpell(count) => repeat_id("CHOOSE_EXPERT_SPELL", *count),
            Self::ExpertSpellFrom(count, trads) => {
                repeat_id(&format!("CHOOSE_EXPERT_SPELL({})", trads.join(",")), *count)
            }
            Self::MasterSpell(count) => repeat_id("CHOOSE_MASTER_SPELL", *count),
            Self::MasterSpellFrom(count, trads) => {
                repeat_id(&format!("CHOOSE_MASTER_SPELL({})", trads.join(",")), *count)
            }
            Self::Select(count, choice_id) => repeat_id(&format!("SELECT='{choice_id}'"), *count),
            Self::Score(count) => repeat_id("CHOOSE_SCORE", *count),
            Self::Slots(target) => vec![format!("CHOOSE_SLOTS='{target}'")],
            Self::MagicTalent(count, trads) => {
                repeat_id(&format!("CHOOSE_MAGIC_TALENT({})", trads.join(",")), *count)
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
    Expr(NumExpr),
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
pub struct CondExpr {
    pub left: CondOperand,
    pub op: CondOperator,
    pub right: CondOperand,
}

impl CondExpr {
    pub fn condition_eval(&self, level: i64, defense: i64) -> bool {
        let left_value = self.left.eval(level, defense);
        let right_value = self.right.eval(level, defense);
        match self.op {
            CondOperator::Lt => left_value < right_value,
            CondOperator::Gt => left_value > right_value,
            CondOperator::LtEq => left_value <= right_value,
            CondOperator::GtEq => left_value >= right_value,
            CondOperator::Eq => left_value == right_value,
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum CondOperand {
    Level,
    Defense,
    Number(i64),
}

impl Display for CondOperand {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Level => f.write_str("LEVEL"),
            Self::Defense => f.write_str("DEF"),
            Self::Number(x) => f.write_str(&x.to_string()),
        }
    }
}

impl CondOperand {
    fn eval(&self, level: i64, defense: i64) -> i64 {
        match self {
            Self::Level => level,
            Self::Defense => defense,
            Self::Number(x) => *x,
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum CondOperator {
    Lt,
    Gt,
    LtEq,
    GtEq,
    Eq,
}

impl Display for CondOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Lt => f.write_char('<'),
            Self::Gt => f.write_char('>'),
            Self::LtEq => f.write_str("<="),
            Self::GtEq => f.write_str(">="),
            Self::Eq => f.write_char('='),
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum Condition {
    None,
    Has(HasCategory),
    If(CondExpr),
}

impl Condition {
    pub fn unconditional(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts")]
pub struct NumExpr {
    pub init: NumOperand,
    pub rest: Vec<(NumOperator, NumOperand)>,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum NumOperand {
    Str,
    Agl,
    Int,
    Will,
    Dmg,
    Level,
    Number(i64),
}

impl Display for NumOperand {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Str => f.write_str("STR"),
            Self::Agl => f.write_str("AGL"),
            Self::Int => f.write_str("INT"),
            Self::Will => f.write_str("WILL"),
            Self::Dmg => f.write_str("DMG"),
            Self::Level => f.write_str("LEVEL"),
            Self::Number(x) => f.write_str(&x.to_string()),
        }
    }
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[ts(export, export_to = "modifiers.ts", tag = "type", content = "data")]
#[serde(tag = "type", content = "data")]
pub enum NumOperator {
    Plus,
    Minus,
    Times,
    Div,
    Mod,
}

impl Display for NumOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Plus => f.write_char('+'),
            Self::Minus => f.write_char('-'),
            Self::Times => f.write_char('*'),
            Self::Div => f.write_char('/'),
            Self::Mod => f.write_char('%'),
        }
    }
}
