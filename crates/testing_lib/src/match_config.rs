use std::path::PathBuf;

use crate::stats::SprtConfig;
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
