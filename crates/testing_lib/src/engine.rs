use std::path::PathBuf;

pub struct EngineConfig {
    pub name: String,
    pub binary_path: PathBuf,
    pub docker_path: Option<PathBuf>,
    pub args: Vec<String>,
}

impl EngineConfig {
    pub fn new(name: String, binary_path: PathBuf, args: Vec<String>) -> Self {
        EngineConfig {
            name,
            binary_path,
            docker_path: None,
            args,
        }
    }

    pub fn with_docker(
        name: String,
        binary_path: PathBuf,
        docker_path: PathBuf,
        args: Vec<String>,
    ) -> Self {
        EngineConfig {
            name,
            binary_path,
            docker_path: Some(docker_path),
            args,
        }
    }
}
