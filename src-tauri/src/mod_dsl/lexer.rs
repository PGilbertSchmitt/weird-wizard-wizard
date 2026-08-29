use logos::{Lexer, Logos};

#[derive(Logos, Debug)]
#[logos(skip r"[ \n\t]")]
pub enum Token {
    #[token("NONE")]
    None,

    #[token("[PERM]")]
    Perm,

    #[token("[CAST")]
    Cast,

    #[token("[CAST!")]
    CastDismiss,

    #[token("]")]
    EmptyCast,

    #[token("1m]")]
    OneMin,

    #[token("1h]")]
    OneHour,

    #[token("4h]")]
    FourHour,

    #[token("8h]")]
    EightHour,

    #[token("24h]")]
    #[token("1d]")]
    OneDay,

    #[token("rest]")]
    Rest,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("GRANT")]
    Grant,

    #[token("LOSE")]
    Lose,

    #[token("OVERRIDE")]
    Override,

    #[token("CHOOSE")]
    Choose,

    #[token("APPLY")]
    Apply,

    #[token("STR")]
    Str,

    #[token("AGL")]
    Agl,

    #[token("INT")]
    Int,

    #[token("WILL")]
    Will,

    #[token("BonusDamage")]
    BonusDamage,

    #[token("Defense")]
    Defense,

    #[token("Health")]
    Health,

    #[token("Immunity")]
    Immunity,

    #[token("Language")]
    Language,

    #[token("MagicTalent")]
    MagicTalent,

    #[token("NatDef")]
    NatDef,

    #[token("NoviceSpell")]
    NoviceSpell,

    #[token("ExpertSpell")]
    ExpertSpell,

    #[token("MasterSpell")]
    MasterSpell,

    #[token("Profession")]
    Profession,

    #[token("Score")]
    Score,

    #[token("Sense")]
    Sense,

    #[token("Speed")]
    Speed,

    #[token("SpeedTrait")]
    SpeedTrait,

    #[token("StatBlock")]
    StatBlock,

    #[token("Merge")]
    Merge,

    #[token("Talent")]
    Talent,

    #[token("Tradition")]
    Tradition,

    #[token("Heal")]
    Heal,

    #[token("SELECT")]
    Select,

    #[token("SELECT_AGAIN")]
    SelectAgain,

    #[token("Slot")]
    Slot,

    #[token("Calc")]
    Calc,

    #[token("Ask")]
    Ask,

    #[regex("[0-9]+[dD][0-9]+", |lex| lex.slice().to_lowercase().to_string(), priority = 5)]
    Dice(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i32>().unwrap(), priority = 4)]
    Number(i32),

    #[token("/")]
    Slash,

    #[regex("[a-zA-Z0-9-_']+", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("@")]
    ConditionMarker,

    #[token("IF")]
    IfCase,

    #[token("HASNOT")]
    HasNotCase,

    #[regex("HAS")]
    HasCase,

    #[token("=")]
    Equals,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token(";")]
    Semicolon,

    #[regex(r#""[^"]+""#, extract_quote_string)]
    #[regex(r#"'[^']+'"#, extract_quote_string)]
    String(String),

    // For conditional expressions
    #[token(">")]
    Gt,

    #[token(">=")]
    GtEq,

    #[token("<")]
    Lt,

    #[token("<=")]
    LtEq,

    #[token("+")]
    Plus,

    #[token("*")]
    Times,
}

fn extract_quote_string(lexer: &mut Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..(slice.len() - 1)].to_string()
}
