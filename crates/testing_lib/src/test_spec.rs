use crate::benchmarks::BenchmarkKind;
use crate::engine::EngineConfig;
use crate::match_config::MatchConfig;

pub struct TestSpec {
    pub baseline: EngineConfig,
    pub candidate: Option<EngineConfig>,
    pub benchmarks: Vec<BenchmarkKind>,
    pub match_config: Option<MatchConfig>,
}

impl TestSpec {
    pub fn full(
        baseline: EngineConfig,
        candidate: EngineConfig,
        benchmarks: Vec<BenchmarkKind>,
        match_config: MatchConfig,
    ) -> Self {
        TestSpec {
            baseline,
            candidate: Some(candidate),
            benchmarks,
            match_config: Some(match_config),
        }
    }
}
