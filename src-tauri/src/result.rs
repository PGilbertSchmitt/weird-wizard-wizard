pub type WWResult<T> = core::result::Result<T, WWError>;

#[derive(Debug)]
pub enum WWError {
    Generic(String),
    Db(sqlx::Error),
    Io(std::io::Error),
    Zip(zip::result::ZipError),
    Csv(csv::Error),
    Tauri(tauri::Error),
}

impl From<sqlx::Error> for WWError {
    fn from(value: sqlx::Error) -> Self {
        WWError::Db(value)
    }
}

impl From<std::io::Error> for WWError {
    fn from(value: std::io::Error) -> Self {
        WWError::Io(value)
    }
}

impl From<csv::Error> for WWError {
    fn from(value: csv::Error) -> Self {
        WWError::Csv(value)
    }
}

impl From<zip::result::ZipError> for WWError {
    fn from(value: zip::result::ZipError) -> Self {
        WWError::Zip(value)
    }
}

impl From<tauri::Error> for WWError {
    fn from(value: tauri::Error) -> Self {
        WWError::Tauri(value)
    }
}

impl std::fmt::Display for WWError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(s) => f.write_str(s),
            Self::Db(e) => write!(f, "DB: {}", e.to_string()),
            Self::Csv(e) => write!(f, "CSV: {}", e.to_string()),
            Self::Io(e) => write!(f, "IO: {}", e.to_string()),
            Self::Zip(e) => write!(f, "IO: {:?}", e),
            Self::Tauri(e) => write!(f, "System: {}", e),
        }
    }
}

impl std::error::Error for WWError {}
