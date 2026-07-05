use std::path::PathBuf;
use tempfile::TempDir;

use crate::{WWResult, ipc::ImportData};

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
