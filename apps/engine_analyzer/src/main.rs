use std::path::PathBuf;

use testing_lib::prelude::*;

fn main() {
    // Baseline config
    let mut baseline_path = PathBuf::new();
    baseline_path.push("target/release/engine");
    let baseline_args: Vec<String> = Vec::new();
    let baseline = EngineConfig::new(String::from("v1"), baseline_path, baseline_args);

    // Candidate config
    let mut candidate_path = PathBuf::new();
    candidate_path.push("target/release/engine");
    let candidate_args: Vec<String> = Vec::new();
    let candidate = EngineConfig::new(String::from("v2"), candidate_path, candidate_args);

    // Sprt config
    let sprt_config = SprtConfig::new(0.0, 5.0, 0.05, 0.05);

    // Match config
    let constraint = Constraint::FixedDepth(2);
    let openings = OpeningSource::StartPos;
    let max_rounds = 100;
    let match_config = MatchConfig::new(constraint, openings, sprt_config, max_rounds);

    // Test spec
    let benchmarks: Vec<BenchmarkKind> = Vec::new();
    let test_spec = TestSpec::full(baseline, candidate, benchmarks, match_config);

    run(&test_spec);
}
