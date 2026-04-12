use std::path::PathBuf;

use crate::stats::SprtConfig;
pub struct MatchConfig {
    pub constraint: Constraint,
    pub openings: OpeningSource,
    pub sprt_config: SprtConfig,
}

pub enum Constraint {
    FixedDepth(u8),
    FixedTime(u64),
    NodeBudget(u64),
    Standard { wtime: u64, btime: u64 },
}
pub enum OpeningSource {
    StartPos,
    Epd(PathBuf),
    Pgn(PathBuf),
}
