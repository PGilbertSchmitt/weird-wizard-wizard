pub fn db_boolean(s: Option<String>) -> bool {
    s.map_or(false, |value| value.to_uppercase() == "TRUE")
}
