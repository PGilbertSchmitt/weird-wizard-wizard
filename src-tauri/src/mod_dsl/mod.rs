mod ast;
mod lexer;
mod parser;

pub fn validate_mod_str(mod_str: Option<&str>) -> Option<String> {
    let mod_str = match mod_str {
        None => return None,
        Some(value) => value,
    };
    if let Err(err) = parser::parse_mods(mod_str) {
        Some(err)
    } else {
        None
    }
}
