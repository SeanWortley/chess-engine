use std::env;
use std::path::PathBuf;

use testing_lib::prelude::*;

fn main() {
    let opening_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/books/noomen.pgn");
    unsafe {
        env::set_var("CUTECHESS_CLI", "/usr/sbin/cutechess-cli");
    }

    // Baseline config
    let mut baseline_path = PathBuf::new();
    baseline_path.push(".engine_archive/linux/v7.1-TTMateScoring");
    let baseline_args: Vec<String> = Vec::new();
    let baseline = EngineConfig::new(String::from("v7.0"), baseline_path, baseline_args);

    // Candidate config
    let mut candidate_path = PathBuf::new();
    candidate_path.push("target/release/engine");
    let candidate_args: Vec<String> = Vec::new();
    let candidate = EngineConfig::new(String::from("v8"), candidate_path, candidate_args);

    // Regression Check Sprt config
    let sprt_config = SprtConfig::new(-10.0, 0.0, 0.05, 0.05);

    // Regression Check Match config
    let constraint = Constraint::FixedMoveTime(100);
    let openings = OpeningSource::Pgn(opening_path.clone());
    let max_rounds = 5000;
    let regression_match_config = MatchConfig::new(constraint, openings, sprt_config, max_rounds);

    // Validation Sprt config
    let sprt_config = SprtConfig::new(0.0, 10.0, 0.05, 0.05);

    // Validation Match config
    let constraint = Constraint::FixedMoveTime(500);
    let openings = OpeningSource::Pgn(opening_path);
    let max_rounds = 5000;
    let validation_match_config = MatchConfig::new(constraint, openings, sprt_config, max_rounds);

    // Regression Test spec
    let regression_test_spec = TestSpec::match_only(
        baseline.clone(),
        candidate.clone(),
        regression_match_config.clone(),
    );

    // Validation Test spec
    let mut benchmarks: Vec<BenchmarkKind> = Vec::new();
    benchmarks.push(BenchmarkKind::PerftSpeed);
    benchmarks.push(BenchmarkKind::NodeCount);
    let validation_test_spec = TestSpec::full(
        baseline.clone(),
        candidate.clone(),
        benchmarks,
        validation_match_config,
    );

    let mut benchmarks: Vec<BenchmarkKind> = Vec::new();
    benchmarks.push(BenchmarkKind::PerftSpeed);
    benchmarks.push(BenchmarkKind::NodeCount);
    let _bench_only_spec = TestSpec::benchmark_comparison(baseline, candidate, benchmarks);

    //run(&bench_only_spec);
    run(&regression_test_spec);
    run(&validation_test_spec);
}
