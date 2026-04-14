use crate::{BenchmarkResult, EngineConfig};

pub enum BenchmarkKind {
    PerftSpeed { depth: u8 },
    NodesEvaluated { depth: u8 },
    MovesPerSecond,
}

pub struct BenchMarkComparison {
    pub kind: BenchmarkKind,
    pub baseline: BenchmarkResult,
    pub candidate: Option<BenchmarkResult>,
}

pub fn perft_speed_test(_engine: &EngineConfig, _depth: u8) -> BenchmarkResult {
    BenchmarkResult::PerftSpeed {
        nodes: 0,
        duration_ms: 0,
        nps: 0,
    }
}
