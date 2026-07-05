use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::Size;

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "path.ts")]
pub struct Ancestry {
    pub id:      u32,
    name:        String,
    descriptor:  Option<String>,
    size:        Size,
    speed:       i32,
    add_health:  i32,
    add_nat_def: i32
}

impl Ancestry {}
