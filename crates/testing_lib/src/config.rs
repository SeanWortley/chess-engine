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

pub struct MatchConfig {
    pub constraint: Constraint,
    pub openings: OpeningSource,
    pub sprt_config: SprtConfig,
    pub max_rounds: u16,
}

pub enum Constraint {
    FixedDepth(u8),
    NodeBudget(u64),
    Standard(f64, f64),
    FixedMoveTime(u64),
}

pub enum OpeningSource {
    StartPos,
    Epd(PathBuf),
    Pgn(PathBuf),
}

impl MatchConfig {
    pub fn new(
        constraint: Constraint,
        openings: OpeningSource,
        sprt_config: SprtConfig,
        max_rounds: u16,
    ) -> Self {
        MatchConfig {
            constraint,
            openings,
            sprt_config,
            max_rounds,
        }
    }
}

pub struct SprtConfig {
    pub elo0: f64,
    pub elo1: f64,
    pub alpha: f64,
    pub beta: f64,
}

impl SprtConfig {
    pub fn new(elo0: f64, elo1: f64, alpha: f64, beta: f64) -> Self {
        SprtConfig {
            elo0,
            elo1,
            alpha,
            beta,
        }
    }
}
