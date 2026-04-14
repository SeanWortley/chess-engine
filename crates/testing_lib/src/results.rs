use crate::benchmarks::BenchMarkComparison;

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
