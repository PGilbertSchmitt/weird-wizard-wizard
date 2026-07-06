mod response;
mod init_seed;
mod run_seed;

pub use init_seed::*;
pub use run_seed::*;
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
