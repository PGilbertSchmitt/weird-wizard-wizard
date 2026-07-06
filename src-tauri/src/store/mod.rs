use std::path::PathBuf;
use tauri::{AppHandle, Wry, Manager, async_runtime::Mutex};
use tempfile::TempDir;

use crate::{WWResult, db, ipc::ImportData};

// For storing non-DB app state
pub struct WWAppData {
    pub import_dir: Option<TempDir>,
    pub import_data: Option<ImportData>,
}

impl WWAppData {
    pub fn new() -> Self {
        Self {
            import_dir: None,
            import_data: None,
        }
    }

    pub fn add_import_dir(&mut self) -> WWResult<PathBuf> {
        let tmp_dir = TempDir::new()?;
        let path = tmp_dir.path().to_owned();
        self.import_dir = Some(tmp_dir);
        Ok(path)
    }

    pub fn drop_import(&mut self) {
        self.import_dir = None;
        self.import_data = None;
    }
}

pub fn get_app_data_state(app: &AppHandle<Wry>) -> WWResult<tauri::State<'_, Mutex<WWAppData>>> {
    Ok(app.state())
}

pub fn get_database(app: &AppHandle<Wry>) -> WWResult<tauri::State<'_, Mutex<db::DatabaseState>>> {
    Ok(app.state())
}
