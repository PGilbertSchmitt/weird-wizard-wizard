use std::path::PathBuf;
use tempfile::TempDir;
use crate::WWResult;

// For storing non-DB app state
pub struct WWAppData {
    pub import_dir: Option<TempDir>,
}

impl WWAppData {
    pub fn new() -> Self {
        Self {
            import_dir: None,
        }
    }

    pub fn add_import_dir(&mut self) -> WWResult<PathBuf> {
        let tmp_dir = TempDir::new()?;
        let path = tmp_dir.path().to_owned();
        self.import_dir = Some(tmp_dir);
        Ok(path)
    }

    pub fn drop_import_dir(&mut self) {
        self.import_dir = None;
    }
}
