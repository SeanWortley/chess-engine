use crate::benchmark::benchmarks::BenchMarkComparison;

pub struct GameResult {
    pub white: String,
    pub black: String,
    pub outcome: GameOutcome,
}

pub enum GameOutcome {
    WhiteWon,
    BlackWon,
    Draw,
}

pub struct MatchResults {
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
}

pub struct TestResult {
    pub match_results: Option<MatchResults>,
    pub benchmark_results: Vec<BenchMarkComparison>,
}

pub enum SprtResult {
    AcceptH0,
    AcceptH1,
    Continue,
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

pub struct BenchmarkComparisonResult {
    pub baseline: BenchmarkResult,
    pub candidate: BenchmarkResult,
}
