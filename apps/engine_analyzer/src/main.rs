use std::env;
use std::path::PathBuf;

use testing_lib::prelude::*;

fn main() {
    unsafe {
        env::set_var("CUTECHESS_CLI", "/usr/sbin/cutechess-cli");
    }

    // Baseline config
    let mut baseline_path = PathBuf::new();
    baseline_path.push("target/release/engine");
    let mut baseline_args: Vec<String> = Vec::new();
    baseline_args.push("--engine=v2".to_string());
    let baseline = EngineConfig::new(String::from("baseline"), baseline_path, baseline_args);

    // Candidate config
    let mut candidate_path = PathBuf::new();
    candidate_path.push("target/release/engine");
    let mut candidate_args: Vec<String> = Vec::new();
    candidate_args.push("--engine=v3".to_string());
    let candidate = EngineConfig::new(String::from("candidate"), candidate_path, candidate_args);

    // Regression Check Sprt config
    let sprt_config = SprtConfig::new(-10.0, 0.0, 0.05, 0.05);

    // Regression Check Match config
    let constraint = Constraint::FixedMoveTime(100);
    let openings = OpeningSource::StartPos;
    let max_rounds = 5000;
    let match_config = MatchConfig::new(constraint, openings, sprt_config, max_rounds);

    // Validation Sprt config
    let sprt_config = SprtConfig::new(0.0, 10.0, 0.05, 0.05);

    // Validation Match config
    let constraint = Constraint::FixedMoveTime(500);
    let openings = OpeningSource::StartPos;
    let max_rounds = 5000;
    let match_config = MatchConfig::new(constraint, openings, sprt_config, max_rounds);

    // Regression Test spec
    let test_spec = TestSpec::match_only(baseline.clone(), candidate.clone(), match_config.clone());

    // Validation Test spec
    let mut benchmarks: Vec<BenchmarkKind> = Vec::new();
    benchmarks.push(BenchmarkKind::PerftSpeed);
    let test_spec = TestSpec::full(baseline, candidate, benchmarks, match_config);

    run(&test_spec);
}
