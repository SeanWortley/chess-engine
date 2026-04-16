pub mod benchmark;
pub mod config;
pub mod match_runner;
pub mod results;
pub mod runner;
pub mod spec;
pub mod uci_runner;

pub use benchmark::{BenchMarkComparison, BenchmarkKind, analysis};
pub use config::{Constraint, EngineConfig, MatchConfig, OpeningSource, SprtConfig};
pub use results::{BenchmarkResult, GameOutcome, GameResult, MatchResults, SprtResult, TestResult};
pub use runner::run;
pub use spec::TestSpec;
pub use uci::{UciCommand, UciMove};

pub mod prelude {
    pub use crate::{
        BenchMarkComparison, BenchmarkKind, BenchmarkResult, Constraint, EngineConfig, GameOutcome,
        GameResult, MatchConfig, MatchResults, OpeningSource, SprtConfig, SprtResult, TestResult,
        TestSpec, run,
    };
}
