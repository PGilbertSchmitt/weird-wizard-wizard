use super::{
    ast::{
        ApplyTarget, ChooseTarget, Condition, Dice, ExprOp, ExprValue, GrantTarget, HasCategory,
        HealAmount, LoseTarget, Modifier, OverrideTarget, SimpleExpr, SlotsTarget, Target,
        WhenDuration, WhenMod,
    },
    lexer::Token,
};
use logos::{Lexer, Logos};

type Tokens<'a> = std::iter::Peekable<Lexer<'a, Token>>;

pub fn parse_mods(input: &str) -> Result<Vec<Modifier>, String> {
    let mut tokens: Tokens = Token::lexer(input).peekable();
    let when = parse_casting(&mut tokens)?.unwrap_or(WhenMod::CastOnce);
    let mut v = Vec::new();
    while tokens.peek().is_some() {
        if let Some(modifier) = parse_single_mod(&mut tokens, when)? {
            v.push(modifier);
        }
    }
    Ok(v)
}

/** Unwrap a token */
fn ut(token: Option<Result<Token, ()>>) -> Result<Token, String> {
    match token {
        Some(tok) => {
            tok.map_err(|_| String::from("Unexpected issue parsing token, invalid grammar"))
        }
        None => Err(String::from("Unexpected end of input")),
    }
}

fn parse_casting(tokens: &mut Tokens) -> Result<Option<WhenMod>, String> {
    match tokens.peek().as_ref() {
        None => Err(String::from("Unexpected end of input")),
        Some(&Err(())) => Err(String::from(
            "Unexpected issue parsing token, invalid grammar",
        )),
        Some(&Ok(Token::None)) => {
            tokens.next();
            Ok(None)
        }
        Some(&Ok(Token::Apply)) => Ok(Some(WhenMod::CastOnce)),
        Some(&Ok(Token::Cast)) => {
            tokens.next();
            Ok(Some(parse_cast_time(tokens)?.map_or_else(
                || WhenMod::CastOnce,
                |dur| WhenMod::CastTime(dur),
            )))
        }
        Some(&Ok(Token::CastDismiss)) => {
            tokens.next();
            Ok(Some(parse_cast_time(tokens)?.map_or_else(
                // In the case that [CAST!] is entered, it's treated the same as [CAST]
                || WhenMod::CastOnce,
                |dur| WhenMod::CastTimeDismiss(dur),
            )))
        }
        Some(&Ok(Token::Perm)) => {
            tokens.next();
            Ok(Some(WhenMod::Permanent))
        }
        Some(&Ok(other_token)) => {
            return Err(format!("Did not expect to find token: '{other_token:?}'"));
        }
    }
}

fn parse_cast_time(tokens: &mut Tokens) -> Result<Option<WhenDuration>, String> {
    match ut(tokens.next())? {
        Token::EmptyCast => Ok(None),
        Token::EightHour => Ok(Some(WhenDuration::EightHours)),
        Token::FourHour => Ok(Some(WhenDuration::FourHours)),
        Token::OneDay => Ok(Some(WhenDuration::OneDay)),
        Token::OneHour => Ok(Some(WhenDuration::OneHour)),
        Token::OneMin => Ok(Some(WhenDuration::OneMinute)),
        Token::Rest => Ok(Some(WhenDuration::Rest)),
        other => Err(format!("Unexpected token {other:?} when parsing CAST")),
    }
}

fn parse_single_mod(tokens: &mut Tokens, when: WhenMod) -> Result<Option<Modifier>, String> {
    let next = tokens.next();
    if next.is_none() {
        return Ok(None);
    }

    let target = match ut(next)? {
        // This eats all semicolons between mods
        Token::Semicolon => return Ok(None),
        Token::Grant => Target::Grant(parse_grant_target(tokens)?),
        Token::Lose => Target::Lose(parse_lose_target(tokens)?),
        Token::Override => Target::Override(parse_override_target(tokens)?),
        Token::Choose => Target::Choose(parse_choose_target(tokens)?),
        Token::Apply => Target::Apply(parse_apply_target(tokens)?),
        other => {
            return Err(format!(
                "Unexpected token '{other:?}', expected one of GRANT, LOSE, OVERRIDE, CHOOSE, or APPLY"
            ));
        }
    };

    let condition = match tokens.peek().as_ref() {
        Some(Ok(Token::ConditionMarker)) => {
            tokens.next();
            parse_condition(tokens)?
        }
        _ => Condition::None,
    };

    Ok(Some(Modifier {
        when,
        target,
        condition,
    }))
}

fn parse_grant_target(tokens: &mut Tokens) -> Result<GrantTarget, String> {
    eat_dot(tokens)?;
    match ut(tokens.next())? {
        // Targets followed by a number
        Token::Str => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Str(parse_number(tokens)?))
        }
        Token::Agl => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Agl(parse_number(tokens)?))
        }
        Token::Int => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Int(parse_number(tokens)?))
        }
        Token::Will => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Will(parse_number(tokens)?))
        }
        Token::Speed => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Speed(parse_number(tokens)?))
        }
        Token::Health => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Health(parse_number(tokens)?))
        }
        Token::Defense => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Defense(parse_number(tokens)?))
        }
        Token::NatDef => {
            eat_dot(tokens)?;
            Ok(GrantTarget::NatDef(parse_number(tokens)?))
        }
        Token::BonusDamage => {
            eat_dot(tokens)?;
            Ok(GrantTarget::BonusDamage(parse_number(tokens)?))
        }

        // Targets followed by a comma-separated list
        Token::Tradition => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Tradition(parse_comma_list(tokens)?))
        }
        Token::Language => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Language(parse_comma_list(tokens)?))
        }
        Token::Sense => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Sense(parse_comma_list(tokens)?))
        }
        Token::Immunity => {
            eat_dot(tokens)?;
            Ok(GrantTarget::Immunity(parse_comma_list(tokens)?))
        }
        Token::SpeedTrait => {
            eat_dot(tokens)?;
            Ok(GrantTarget::SpeedTrait(parse_comma_list(tokens)?))
        }

        // Targets with a sub-category
        Token::Talent => {
            eat_dot(tokens)?;
            let category = parse_ident(tokens)?;
            eat_dot(tokens)?;
            let subcategory = parse_ident(tokens)?;
            Ok(GrantTarget::Talent(category, subcategory))
        }
        Token::MagicTalent => {
            eat_dot(tokens)?;
            let category = parse_ident(tokens)?;
            eat_dot(tokens)?;
            let subcategory = parse_ident(tokens)?;
            Ok(GrantTarget::MagicTalent(category, subcategory))
        }

        other_token => {
            return Err(format!(
                "Expected a GRANT target, instead found '{other_token:?}'"
            ));
        }
    }
}

fn parse_lose_target(tokens: &mut Tokens) -> Result<LoseTarget, String> {
    eat_dot(tokens)?;
    match ut(tokens.next())? {
        Token::Talent => {
            eat_dot(tokens)?;
            let category = parse_ident(tokens)?;
            eat_dot(tokens)?;
            let subcategory = parse_ident(tokens)?;
            Ok(LoseTarget::Talent(category, subcategory))
        }
        Token::MagicTalent => {
            eat_dot(tokens)?;
            let category = parse_ident(tokens)?;
            eat_dot(tokens)?;
            let subcategory = parse_ident(tokens)?;
            Ok(LoseTarget::MagicTalent(category, subcategory))
        }

        other_token => {
            return Err(format!(
                "Expected a LOSE target, instead found '{other_token:?}'"
            ));
        }
    }
}

fn parse_override_target(tokens: &mut Tokens) -> Result<OverrideTarget, String> {
    eat_dot(tokens)?;
    match ut(tokens.next())? {
        Token::Speed => {
            eat_dot(tokens)?;
            Ok(OverrideTarget::Speed(parse_number(tokens)?))
        }
        Token::Defense => {
            eat_dot(tokens)?;
            Ok(OverrideTarget::Defense(parse_number(tokens)?))
        }
        Token::StatBlock => {
            eat_dot(tokens)?;
            Ok(OverrideTarget::StatBlock(parse_ident(tokens)?))
        }

        other_token => Err(format!(
            "Expected an OVERRIDE target, instead found '{other_token:?}'"
        )),
    }
}

fn parse_choose_target(tokens: &mut Tokens) -> Result<ChooseTarget, String> {
    eat_dot(tokens)?;
    let target_token = ut(tokens.next())?;
    let multiplier: u32 = parse_multiplier(tokens)?.try_into().unwrap();

    match target_token {
        Token::Language => Ok(ChooseTarget::Language(multiplier)),
        Token::Profession => Ok(ChooseTarget::Profession(multiplier)),
        Token::Tradition => Ok(ChooseTarget::Tradition(multiplier)),
        Token::NoviceSpell => match parse_choose_spell_subcategory(tokens)? {
            Some(subcats) => Ok(ChooseTarget::NoviceSpellFrom(multiplier, subcats)),
            None => Ok(ChooseTarget::NoviceSpell(multiplier)),
        },
        Token::ExpertSpell => match parse_choose_spell_subcategory(tokens)? {
            Some(subcats) => Ok(ChooseTarget::ExpertSpellFrom(multiplier, subcats)),
            None => Ok(ChooseTarget::ExpertSpell(multiplier)),
        },
        Token::MasterSpell => match parse_choose_spell_subcategory(tokens)? {
            Some(subcats) => Ok(ChooseTarget::MasterSpellFrom(multiplier, subcats)),
            None => Ok(ChooseTarget::MasterSpell(multiplier)),
        },

        Token::Select => {
            eat_equal_sign(tokens)?;
            Ok(ChooseTarget::Select(multiplier, parse_ident(tokens)?))
        }

        Token::SelectAgain => {
            eat_equal_sign(tokens)?;
            Ok(ChooseTarget::SelectAgain(multiplier, parse_ident(tokens)?))
        }

        Token::Score => Ok(ChooseTarget::Score(multiplier)),

        other_token => Err(format!(
            "Expected a CHOOSE target, instead found '{other_token:?}'"
        )),
    }
}

fn parse_apply_target(tokens: &mut Tokens) -> Result<ApplyTarget, String> {
    eat_dot(tokens)?;
    match ut(tokens.next())? {
        Token::Heal => {
            let amt = parse_apply_heal_info(tokens)?;
            Ok(ApplyTarget::Heal(amt))
        }
        Token::Health => {
            let amt = parse_apply_heal_info(tokens)?;
            Ok(ApplyTarget::Health(amt))
        }

        Token::Slot => {
            let target = parse_slot_target(tokens)?;
            Ok(ApplyTarget::Slots(target))
        }
        other_token => Err(format!(
            "Expected an APPLY target, instead found '{other_token:?}'"
        )),
    }
}

fn parse_condition(tokens: &mut Tokens) -> Result<Condition, String> {
    match ut(tokens.next())? {
        Token::HasNotCase => Ok(Condition::HasNot),
        Token::HasCase => Ok(Condition::Has(parse_has_case(tokens)?)),
        Token::IfCase => Ok(Condition::If(parse_simple_expression(tokens)?)),

        other_token => Err(format!(
            "Expected a Condition type, instead found '{other_token:?}'"
        )),
    }
}

fn eat_dot(tokens: &mut Tokens) -> Result<(), String> {
    match ut(tokens.next())? {
        Token::Dot => Ok(()),
        other_token => Err(format!(
            "Expected a dot '.', instead found '{other_token:?}'"
        )),
    }
}

fn eat_close_paren(tokens: &mut Tokens) -> Result<(), String> {
    match ut(tokens.next())? {
        Token::CloseParen => Ok(()),
        other_token => Err(format!(
            "Expected a close paren ')', instead found '{other_token:?}'"
        )),
    }
}

fn eat_open_paren(tokens: &mut Tokens) -> Result<(), String> {
    match ut(tokens.next())? {
        Token::OpenParen => Ok(()),
        other_token => Err(format!(
            "Expected an open paren '(', instead found '{other_token:?}'"
        )),
    }
}

fn eat_equal_sign(tokens: &mut Tokens) -> Result<(), String> {
    match ut(tokens.next())? {
        Token::Equals => Ok(()),
        other_token => Err(format!(
            "Expected an equal sign '=', instead found '{other_token:?}'"
        )),
    }
}

fn parse_number(tokens: &mut Tokens) -> Result<i32, String> {
    match ut(tokens.next())? {
        Token::Number(v) => Ok(v),
        other_token => Err(format!("Expected number, instead found '{other_token:?}'")),
    }
}

fn parse_ident(tokens: &mut Tokens) -> Result<String, String> {
    match ut(tokens.next())? {
        Token::Identifier(s) => Ok(s),
        // A string is acceptable in most places where an ident is expected
        Token::String(s) => Ok(s),
        other_token => Err(format!(
            "Expected identifier, instead found '{other_token:?}'"
        )),
    }
}

fn parse_string(tokens: &mut Tokens) -> Result<String, String> {
    match ut(tokens.next())? {
        Token::String(s) => Ok(s),
        other_token => Err(format!("Expected string, instead found '{other_token:?}'")),
    }
}

fn parse_comma_list(tokens: &mut Tokens) -> Result<Vec<String>, String> {
    let mut identifiers = Vec::new();

    identifiers.push(parse_ident(tokens)?);

    while let Some(Ok(Token::Comma)) = tokens.peek().as_ref() {
        tokens.next();
        identifiers.push(parse_ident(tokens)?);
    }

    Ok(identifiers)
}

fn parse_multiplier(tokens: &mut Tokens) -> Result<i32, String> {
    match tokens.peek().as_ref() {
        Some(&Err(())) => Err(String::from(
            "Unexpected issue parsing token, invalid grammar",
        )),
        Some(&Ok(Token::OpenParen)) => {
            tokens.next();
            let value = parse_number(tokens)?;
            eat_close_paren(tokens)?;
            Ok(value)
        }
        // The multiplier is optional, so if it's missing, then it defaults to a value of 1,
        // and no tokens are consumed.
        _ => Ok(1),
    }
}

fn parse_choose_spell_subcategory(tokens: &mut Tokens) -> Result<Option<Vec<String>>, String> {
    match tokens.peek().as_ref() {
        Some(&Ok(Token::Dot)) => {
            eat_dot(tokens)?;
            Ok(Some(parse_comma_list(tokens)?))
        }
        _ => Ok(None),
    }
}

fn parse_apply_heal_info(tokens: &mut Tokens) -> Result<HealAmount, String> {
    eat_dot(tokens)?;
    match ut(tokens.next())? {
        Token::Number(amount) => Ok(HealAmount::Number(amount)),
        Token::Dice(roll_str) => Ok(HealAmount::Roll(parse_dice(roll_str)?)),
        Token::Calc => Ok(HealAmount::Expr(parse_simple_expression(tokens)?)),
        Token::Ask => {
            eat_open_paren(tokens)?;
            let s = parse_string(tokens)?;
            eat_close_paren(tokens)?;
            Ok(HealAmount::Ask(s))
        }
        other_token => Err(format!(
            "Expected APPLY HEAL/HEALTH target, instead found '{other_token:?}'"
        )),
    }
}

fn parse_dice(roll_str: String) -> Result<Dice, String> {
    let parts = roll_str.split("d").collect::<Vec<&str>>();
    let die_count = parts[0].parse().unwrap();
    let die_size = parts[1].parse().unwrap();

    if die_count < 1 {
        Err(String::from("Expected ROLL to have positive die count"))
    } else if die_size < 1 {
        Err(String::from("Expected ROLL to have positive die size"))
    } else {
        Ok(Dice {
            die_count,
            die_size,
        })
    }
}

#[derive(Debug)]
struct TmpExpr {
    left: Option<ExprValue>,
    op: Option<ExprOp>,
    right: Option<ExprValue>,
}

impl TmpExpr {
    fn new() -> Self {
        Self {
            left: None,
            op: None,
            right: None,
        }
    }

    fn push_value(&mut self, value: ExprValue) -> Result<(), String> {
        if self.left.is_none() {
            self.left = Some(value);
            Ok(())
        } else if self.right.is_none() {
            if self.op.is_some() {
                self.right = Some(value);
                Ok(())
            } else {
                Err(format!(
                    "Expected operator while parsing expression, received value '{}'",
                    value.to_str()
                ))
            }
        } else {
            Err(format!(
                "Unexpected value while parsing expression, received value '{}'",
                value.to_str()
            ))
        }
    }

    fn push_op(&mut self, op: ExprOp) -> Result<(), String> {
        if self.left.is_some() && self.op.is_none() {
            self.op = Some(op);
            Ok(())
        } else {
            Err(format!(
                "Expected value while parsing expression, received operator '{}'",
                op.to_str()
            ))
        }
    }

    fn to_expr(self) -> Result<SimpleExpr, String> {
        if self.left.is_none() {
            return Err(String::from("Empty expression, shame on you"));
        }
        let left = self.left.unwrap();

        if self.right.is_none() && self.op.is_some() {
            return Err(String::from(
                "Unterminated expression, it shouldn't end with an operator",
            ));
        }

        if self.right.is_some() {
            Ok(SimpleExpr { left, right: None })
        } else {
            Ok(SimpleExpr {
                left,
                right: Some((self.op.unwrap(), self.right.unwrap())),
            })
        }
    }
}

// The calculation expression language is very minimal. No pemdas, expressions are ordered
fn parse_simple_expression(tokens: &mut Tokens) -> Result<SimpleExpr, String> {
    eat_open_paren(tokens)?;
    let mut tmp_expr = TmpExpr::new();

    loop {
        let token = ut(tokens.next())?;
        match token {
            Token::CloseParen => break,

            // Operand values
            Token::Identifier(text) => {
                let match_text = text.to_uppercase();
                let value = match match_text.as_str() {
                    "DEF" => ExprValue::Defense,
                    "DMG" => ExprValue::Dmg,
                    "LEVEL" => ExprValue::Level,
                    "HEALTH" => ExprValue::Health,
                    _ => {
                        return Err(format!(
                            "Unknown identifier '{text}' while parsing expression"
                        ));
                    }
                };
                tmp_expr.push_value(value)?;
            }
            Token::Number(value) => {
                let num = ExprValue::Number(value);
                tmp_expr.push_value(num)?;
            }

            // Operators
            Token::Equals => {
                tmp_expr.push_op(ExprOp::Eq)?;
            }
            Token::Gt => {
                tmp_expr.push_op(ExprOp::Gt)?;
            }
            Token::GtEq => {
                tmp_expr.push_op(ExprOp::GtEq)?;
            }
            Token::Lt => {
                tmp_expr.push_op(ExprOp::Lt)?;
            }
            Token::LtEq => {
                tmp_expr.push_op(ExprOp::LtEq)?;
            }
            Token::Slash => {
                tmp_expr.push_op(ExprOp::Div)?;
            }

            other_token => {
                return Err(format!(
                    "Unexpected token '{other_token:?}' while parsing expression"
                ));
            }
        };
    }

    tmp_expr.to_expr()
}

fn parse_slot_target(tokens: &mut Tokens) -> Result<SlotsTarget, String> {
    eat_dot(tokens)?;
    let target_token = ut(tokens.next())?;
    let multiplier: u32 = parse_multiplier(tokens)?.try_into().unwrap();
    match target_token {
        Token::NoviceSpell => Ok(SlotsTarget::NoviceSpell(multiplier)),
        Token::ExpertSpell => Ok(SlotsTarget::ExpertSpell(multiplier)),
        Token::MasterSpell => Ok(SlotsTarget::MasterSpell(multiplier)),
        Token::Identifier(tradition_ident) => {
            if tradition_ident.to_uppercase() == "ANY" {
                return Ok(SlotsTarget::AnySpell(multiplier));
            }

            // If it's not the string "ANY", then it's a tradition (as long as there's no tradition
            // called "ANY")

            if let Some(&Ok(Token::Dot)) = tokens.peek().as_deref() {
                eat_dot(tokens)?;

                let spell_ident = parse_ident(tokens)?;
                Ok(SlotsTarget::SpecificSpell(
                    multiplier,
                    tradition_ident,
                    spell_ident,
                ))
            } else {
                Ok(SlotsTarget::Tradition(multiplier, tradition_ident))
            }
        }

        other_token => Err(format!(
            "Expected a SLOT target, instead found '{other_token:?}'"
        )),
    }
}

fn parse_has_case(tokens: &mut Tokens) -> Result<HasCategory, String> {
    eat_dot(tokens)?;
    match ut(tokens.next())? {
        Token::Sense => {
            eat_dot(tokens)?;
            Ok(HasCategory::Sense(parse_ident(tokens)?))
        }
        Token::SpeedTrait => {
            eat_dot(tokens)?;
            Ok(HasCategory::SpeedTrait(parse_ident(tokens)?))
        }

        other_token => Err(format!(
            "Expected a HAS condition target, instead found '{other_token:?}'"
        )),
    }
}
