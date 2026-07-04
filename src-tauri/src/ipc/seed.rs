use std::{fs::File, sync::Mutex};

use crate::{WWError, store::WWAppData};

use super::response::{IpcResponse, IpcResult};
use tauri::{AppHandle, Manager, Wry, command};

#[command]
pub async fn init_seed(app: AppHandle<Wry>, filepath: String) -> IpcResult<()> {
    let state: tauri::State<'_, Mutex<WWAppData>> = app.state();
    let mut state = state.lock().unwrap();
    // println!("Old filepath is {:?}, but we gotted a new one called '{filepath}'", state.foo);

    // let file = open_file(&filepath).map_err(|e| IpcError::from(e))?;
    let file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => return IpcResult::Err(WWError::Io(e).into()),

    };
    let tmp_dir_path = state.add_import_dir().unwrap();
    
    let mut zipfile = zip::ZipArchive::new(file).unwrap();
    zipfile.extract(&tmp_dir_path).unwrap();
    println!("Extracted contents to {:?}, not dropping.", tmp_dir_path);
    // state.drop_import_dir();
    IpcResult::Ok(IpcResponse { data: () })
}

// fn open_file(filepath: &str) -> WWResult<File> {
//     match File::open(filepath) {
//         Ok(f) => Ok(f),
//         Err(e) => Err(e.into()),
//     }
// }
