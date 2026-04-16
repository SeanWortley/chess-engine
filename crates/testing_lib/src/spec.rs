use crate::benchmark::benchmarks::BenchmarkKind;
use crate::config::EngineConfig;
use crate::config::MatchConfig;

pub struct TestSpec {
    mode: TestMode,
    baseline: EngineConfig,
    candidate: Option<EngineConfig>,
    benchmarks: Vec<BenchmarkKind>,
    match_config: Option<MatchConfig>,
}

pub enum TestMode {
    BenchmarkSolo,
    BenchmarkComparison,
    Match,
    Full,
}

impl TestSpec {
    pub fn benchmark_solo(engine: EngineConfig, benchmarks: Vec<BenchmarkKind>) -> Self {
        TestSpec {
            mode: TestMode::BenchmarkSolo,
            baseline: engine,
            candidate: None,
            benchmarks,
            match_config: None,
        }
    }

    pub fn benchmark_comparison(
        baseline: EngineConfig,
        candidate: EngineConfig,
        benchmarks: Vec<BenchmarkKind>,
    ) -> Self {
        TestSpec {
            mode: TestMode::BenchmarkComparison,
            baseline,
            candidate: Some(candidate),
            benchmarks,
            match_config: None,
        }
    }

    pub fn match_only(
        baseline: EngineConfig,
        candidate: EngineConfig,
        match_config: MatchConfig,
    ) -> Self {
        TestSpec {
            mode: TestMode::Match,
            baseline,
            candidate: Some(candidate),
            benchmarks: Vec::new(),
            match_config: Some(match_config),
        }
    }

    pub fn full(
        baseline: EngineConfig,
        candidate: EngineConfig,
        benchmarks: Vec<BenchmarkKind>,
        match_config: MatchConfig,
    ) -> Self {
        TestSpec {
            mode: TestMode::Full,
            baseline,
            candidate: Some(candidate),
            benchmarks,
            match_config: Some(match_config),
        }
    }

    pub fn mode(&self) -> &TestMode {
        &self.mode
    }

    pub fn baseline(&self) -> &EngineConfig {
        &self.baseline
    }

    pub fn candidate(&self) -> Option<&EngineConfig> {
        self.candidate.as_ref()
    }

    pub fn benchmarks(&self) -> &[BenchmarkKind] {
        &self.benchmarks
    }

    pub fn match_config(&self) -> Option<&MatchConfig> {
        self.match_config.as_ref()
    }
}
