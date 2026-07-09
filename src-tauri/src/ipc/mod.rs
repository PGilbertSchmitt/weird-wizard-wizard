mod ancestries;
mod import;
mod info_tables;
mod response;

pub use ancestries::*;
pub use import::*;
pub use info_tables::*;

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
