mod response;
mod import;

pub use import::*;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::{WWResult, ipc::response::IpcResult};

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

pub fn emit<T>(
    app: &AppHandle,
    channel: EmitChannel,
    payload: &IpcResult<T>
) -> WWResult<()>
where
    T: Serialize
{
    Ok(app.emit(channel.into(), &payload)?)
}
