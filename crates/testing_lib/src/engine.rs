use std::path::PathBuf;

pub struct EngineConfig {
    pub name: String,
    pub binary: PathBuf,
    pub dockerfile: Option<PathBuf>,
    pub args: Vec<String>,
}
