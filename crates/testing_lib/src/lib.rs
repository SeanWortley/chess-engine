pub mod analysis;
pub mod benchmark_runner;
pub mod benchmarks;
pub mod engine;
pub mod match_config;
pub mod match_runner;
pub mod results;
pub mod stats;
pub mod test_runner;
pub mod test_spec;

pub use analysis::MatchAnalysis;
pub use benchmarks::{BenchMarkComparison, BenchmarkKind};
pub use engine::EngineConfig;
pub use match_config::{Constraint, MatchConfig, OpeningSource};
pub use results::{BenchmarkResult, GameOutcome, GameResult, MatchResults, SprtResult, TestResult};
pub use stats::SprtConfig;
pub use test_runner::run;
pub use test_spec::TestSpec;
pub use uci::{UciCommand, UciMove};

pub mod prelude {
    pub use crate::{
        BenchMarkComparison, BenchmarkKind, BenchmarkResult, Constraint, EngineConfig, GameOutcome,
        GameResult, MatchAnalysis, MatchConfig, MatchResults, OpeningSource, SprtConfig,
        SprtResult, TestResult, TestSpec, run,
    };
}
