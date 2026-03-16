use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveAction {
    pub source: PathBuf,
    pub destination: PathBuf,
}
