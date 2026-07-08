use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::WWError;

#[derive(TS, Debug, Serialize, Deserialize, sqlx::Type)]
#[ts(export, export_to = "etc.ts")]
#[sqlx(type_name = "TEXT")]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl TryFrom<&str> for Size {
    type Error = WWError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "sm" => Ok(Self::Sm),
            "md" => Ok(Self::Md),
            "lg" => Ok(Self::Lg),
            sz => Err(WWError::Generic(format!("{} is not a valid size", sz))),
        }
    }
}