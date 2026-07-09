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
