use crate::{BenchmarkKind, BenchmarkResult, TestSpec, benchmarks};

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

pub fn run_comparison(_test_spec: &TestSpec) -> Vec<BenchmarkResult> {
    Vec::new()
}
