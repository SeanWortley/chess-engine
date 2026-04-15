use crate::{BenchmarkResult, EngineConfig, uci_runner};

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

pub fn perft_speed_test(engine: &EngineConfig, depth: u8) -> BenchmarkResult {
    let _result = uci_runner::run_perft_speed(engine, depth);

    BenchmarkResult::PerftSpeed {
        nodes: 0,
        duration_ms: 0,
        nps: 0,
    }
}
