pub fn db_boolean(s: Option<String>) -> bool {
    s.map_or(false, |value| value.to_uppercase() == "TRUE")
}

pub fn clamp(value: i64, min: i64, max: i64) -> i64 {
    if value <= min {
        min
    } else if value >= max {
        max
    } else {
        value
    }
}
