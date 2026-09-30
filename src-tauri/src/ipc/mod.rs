mod ancestries;
mod characters;
mod choices;
mod import;
mod languages;
mod magic;
mod paths;
mod professions;
mod response;
mod tables;

pub use ancestries::*;
pub use characters::*;
pub use choices::*;
pub use import::*;
pub use languages::*;
pub use magic::*;
pub use paths::*;
pub use professions::*;
pub use tables::*;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::{ipc::response::IpcResult, WWResult};

pub enum EmitChannel {
    IMPORT,
}

impl From<EmitChannel> for &'static str {
    fn from(value: EmitChannel) -> Self {
        match value {
            EmitChannel::IMPORT => "IMPORT",
        }
    }
}

pub fn emit<T>(app: &AppHandle, channel: EmitChannel, payload: &IpcResult<T>) -> WWResult<()>
where
    T: Serialize,
{
    Ok(app.emit(channel.into(), &payload)?)
}
