use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
}
