use crate::benchmarks::BenchmarkKind;
use crate::engine::EngineConfig;
use crate::match_config::MatchConfig;

pub struct TestSpec {
    pub baseline: EngineConfig,
    pub candidate: Option<EngineConfig>,
    pub benchmarks: Vec<BenchmarkKind>,
    pub match_config: Option<MatchConfig>,
}
