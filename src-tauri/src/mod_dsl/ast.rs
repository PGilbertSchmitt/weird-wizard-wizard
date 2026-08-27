#[derive(Debug)]
pub struct Modifier {
    pub when: WhenMod,
    pub target: Target,
    pub condition: Condition,
}

#[derive(Debug)]
pub enum Target {
    Grant(GrantTarget),
    Lose(LoseTarget),
    Override(OverrideTarget),
    Choose(ChooseTarget),
    Apply(ApplyTarget),
}

#[derive(Debug, Clone, Copy)]
pub enum WhenDuration {
    OneMinute,
    OneHour,
    FourHours,
    EightHours,
    OneDay,
    Rest,
}

#[derive(Debug, Clone, Copy)]
pub enum WhenMod {
    Permanent,
    CastOnce,
    CastTime(WhenDuration),
    CastTimeDismiss(WhenDuration),
}

// Grant - Everything except StatBlock, Heal, and Slots
#[derive(Debug)]
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
    Sense(Vec<String>),
    Immunity(Vec<String>),
    SpeedTrait(Vec<String>),
    Talent(String, String),
    MagicTalent(String, String),
}

// Lose - Only Talent and MagicTalent
#[derive(Debug)]
pub enum LoseTarget {
    Talent(String, String),
    MagicTalent(String, String),
}

// Override - Only Speed, Defense, and the unique StatBlock target
#[derive(Debug)]
pub enum OverrideTarget {
    Speed(i32),
    Defense(i32),
    StatBlock(String),
}

// Choose - This MOD uses its own targets, which don't overlap with any of the others
#[derive(Debug)]
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
    SelectAgain(u32, String),
    Score(u32),
}

// Apply - Only Heal, Health, and Slots
#[derive(Debug)]
pub enum ApplyTarget {
    Heal(HealAmount),
    Health(HealAmount),
    Slots(SlotsTarget),
}

#[derive(Debug)]
pub enum HealAmount {
    Number(i32),
    Expr(SimpleExpr),
    Roll(Dice),
    Ask(String),
}

#[derive(Debug)]
pub enum SlotsTarget {
    AnySpell(u32),
    NoviceSpell(u32),
    ExpertSpell(u32),
    MasterSpell(u32),
    Tradition(u32, String),
    SpecificSpell(u32, String, String),
}

#[derive(Debug)]
pub struct Dice {
    pub die_size: i32,
    pub die_count: i32,
}

#[derive(Debug)]
pub enum HasCategory {
    SpeedTrait(String),
    Sense(String),
}

#[derive(Debug)]
pub struct SimpleExpr {
    pub left: ExprValue,
    pub right: Option<(ExprOp, ExprValue)>,
}

#[derive(Debug)]
pub enum ExprValue {
    Health,
    Dmg,
    Level,
    Defense,
    Number(i32),
}

impl ExprValue {
    pub fn to_str(&self) -> String {
        match self {
            Self::Health => String::from("Health"),
            Self::Dmg => String::from("Dmg"),
            Self::Level => String::from("Level"),
            Self::Defense => String::from("Defense"),
            Self::Number(x) => x.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ExprOp {
    Lt,
    Gt,
    LtEq,
    GtEq,
    Eq,
    Div,
}

impl ExprOp {
    pub fn to_str(&self) -> String {
        match self {
            Self::Lt => String::from("<"),
            Self::Gt => String::from(">"),
            Self::LtEq => String::from("<="),
            Self::GtEq => String::from(">="),
            Self::Eq => String::from("="),
            Self::Div => String::from("/"),
        }
    }
}

#[derive(Debug)]
pub enum Condition {
    None,
    HasNot,
    Has(HasCategory),
    If(SimpleExpr),
}

#[derive(Debug)]
pub struct ModOptions {
    pub when: WhenMod,
    pub condition: Condition,
}
