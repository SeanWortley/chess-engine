use crate::BenchmarkResult;

pub enum BenchmarkKind {
    PerftSpeed,
    NodesEvaluated,
    MovesPerSecond,
}

pub struct BenchMarkComparison {
    pub kind: BenchmarkKind,
    pub baseline: BenchmarkResult,
    pub candidate: Option<BenchmarkResult>,
}
