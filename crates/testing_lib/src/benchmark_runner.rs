use crate::{
    BenchmarkKind, BenchmarkResult, TestSpec, benchmarks, results::BenchmarkComparisonResult,
};

pub fn run_solo(test_spec: &TestSpec) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    for benchmark in test_spec.benchmarks() {
        match benchmark {
            BenchmarkKind::PerftSpeed { depth } => {
                let result = benchmarks::perft_speed_test(test_spec.baseline(), *depth);
                results.push(result);
            }
            _ => {}
        }
    }
    results
}

pub fn run_comparison(test_spec: &TestSpec) -> Vec<BenchmarkComparisonResult> {
    let mut results = Vec::new();
    for benchmark in test_spec.benchmarks() {
        match benchmark {
            BenchmarkKind::PerftSpeed { depth } => {
                let baseline_result = benchmarks::perft_speed_test(test_spec.baseline(), *depth);
                let candidate_result =
                    benchmarks::perft_speed_test(test_spec.candidate().unwrap(), *depth);

                results.push(BenchmarkComparisonResult {
                    baseline: baseline_result,
                    candidate: candidate_result,
                });
            }
            _ => {}
        }
    }
    results
}
