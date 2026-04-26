use crate::BenchmarkResult;

pub enum BenchmarkKind {
    PerftSpeed,
    NodeCount,
    MovesPerSecond,
}

pub struct BenchMarkComparison {
    pub kind: BenchmarkKind,
    pub baseline: BenchmarkResult,
    pub candidate: Option<BenchmarkResult>,
}
