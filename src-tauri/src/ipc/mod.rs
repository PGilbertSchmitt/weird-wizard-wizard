mod ancestries;
mod import;
mod magic;
mod response;
mod tables;

pub use ancestries::*;
pub use import::*;
pub use magic::*;
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
