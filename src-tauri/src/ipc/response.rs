use crate::{WWError, WWResult};
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
pub struct IpcError {
    message: String,
}

impl From<WWError> for IpcError {
    fn from(value: WWError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct IpcResponse<T>
where
    T: Serialize,
{
    pub data: T,
}

#[derive(Serialize)]
pub enum IpcResult<T>
where
    T: Serialize,
{
    Ok(IpcResponse<T>),
    Err(IpcError),
}

impl<T> From<WWResult<T>> for IpcResult<T>
where
    T: Serialize,
{
    fn from(value: WWResult<T>) -> Self {
        match value {
            Ok(data) => IpcResult::Ok(IpcResponse { data }),
            Err(err) => IpcResult::Err(IpcError {
                message: err.to_string(),
            }),
        }
    }
}

impl From<WWError> for IpcResult<()> {
    fn from(err: WWError) -> Self {
        IpcResult::Err(err.into())
    }
}
