pub mod analysis;
pub mod benchmarks;
pub mod engine;
pub mod match_config;
pub mod match_runner;
pub mod results;
pub mod stats;
pub mod test_spec;

pub use analysis::MatchAnalysis;
pub use benchmarks::{BenchMarkComparison, BenchmarkKind, BenchmarkResult};
pub use engine::EngineConfig;
pub use match_config::{Constraint, MatchConfig, OpeningSource};
pub use match_runner::run;
pub use results::{GameOutcome, GameResult, MatchResults, SprtResult, TestResult};
pub use stats::SprtConfig;
pub use test_spec::TestSpec;

pub mod prelude {
	pub use crate::{
		BenchMarkComparison, BenchmarkKind, BenchmarkResult, Constraint, EngineConfig,
		GameOutcome, GameResult, MatchAnalysis, MatchConfig, MatchResults, OpeningSource,
		SprtConfig, SprtResult, TestResult, TestSpec, run,
	};
}