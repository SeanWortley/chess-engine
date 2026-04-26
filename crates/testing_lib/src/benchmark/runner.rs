use crate::{
    BenchmarkKind, BenchmarkResult, TestSpec,
    benchmark::{node_count, perft},
    results::BenchmarkComparisonResult,
};

pub fn run_solo(test_spec: &TestSpec) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    for benchmark in test_spec.benchmarks() {
        match benchmark {
            BenchmarkKind::PerftSpeed => {
                let result = perft::perft_speed_test(test_spec.baseline());
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
            BenchmarkKind::PerftSpeed => {
                let baseline_result = perft::perft_speed_test(test_spec.baseline());
                let candidate_result = perft::perft_speed_test(test_spec.candidate().unwrap());

                results.push(BenchmarkComparisonResult {
                    baseline: baseline_result,
                    candidate: candidate_result,
                });
            }
            BenchmarkKind::NodeCount => {
                let baseline_result = node_count::node_count(test_spec.baseline());
                let candidate_result = node_count::node_count(test_spec.candidate().unwrap());

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
