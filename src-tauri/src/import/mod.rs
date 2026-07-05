mod ancestries;

pub(super) use ancestries::*;

pub fn pipe_separate(column: Option<String>) -> Vec<String> {
    match column {
        Some(s) => s.split("|").map(|part| part.trim().to_owned()).collect(),
        None => Vec::new()
    }
}
