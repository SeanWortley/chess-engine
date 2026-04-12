pub enum BenchmarkKind {
    PerftSpeed { depth: u8 },
    NodesEvaluated { depth: u8 },
    MovesPerSecond,
}

pub enum BenchmarkResult {
    PerftSpeed {
        nodes: u64,
        duration_ms: u64,
        nps: u64,
    },
    NodesEvaluated {
        nodes: u64,
    },
    MovesPerSecond {
        moves_per_second: u64,
    },
}

pub struct BenchMarkComparison {
    pub kind: BenchmarkKind,
    pub baseline: BenchmarkResult,
    pub candidate: Option<BenchmarkResult>,
}
