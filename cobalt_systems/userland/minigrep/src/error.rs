use std::fmt;

#[derive(Debug)]
pub enum MinigrepError {
    Io(std::io::Error),
    InvalidCommand(String),
    FileNotFound(String),
    PermissionDenied(String),
    ParseError(String),
    Unknown(String),
}

impl fmt::Display for MinigrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinigrepError::Io(e) => write!(f, "I/O error: {}", e),
            MinigrepError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            MinigrepError::FileNotFound(path) => write!(f, "File not found: {}", path),
            MinigrepError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            MinigrepError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MinigrepError::Unknown(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for MinigrepError {
    fn from(err: std::io::Error) -> MinigrepError {
        MinigrepError::Io(err)
    }
}