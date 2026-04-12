use crate::{
    engine::EngineConfig,
    match_config::MatchConfig,
    results::MatchResults,
    test_spec::{self, TestSpec},
};

pub fn run(test_spec: &TestSpec) -> MatchResults {
    let baseline = test_spec.baseline;
    let candidate = test_spec.baseline;
    let config = test_spec.match_config;
    let args = build_args(&baseline, &candidate, &config.unwrap());
}

fn build_args(baseline: &EngineConfig, candidate: &EngineConfig, config: &MatchConfig) {
    let mut args = Vec::new();

    args.push(format!("-engine"));
    args.push(format!(
        "name={} cmd={} proto=uci",
        baseline.name,
        baseline.binary.to_str().unwrap()
    ))
}
