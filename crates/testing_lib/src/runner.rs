use crate::{
    benchmark::{analysis as benchmark_analyzer, runner as benchmark_runner},
    match_runner,
    spec::{TestMode, TestSpec},
};

pub fn run(test_spec: &TestSpec) {
    match test_spec.mode() {
        TestMode::BenchmarkSolo => {
            let _result = benchmark_runner::run_solo(test_spec);
        }
        TestMode::BenchmarkComparison => {
            let _result = benchmark_runner::run_comparison(test_spec);
        }
        TestMode::Match => {
            match_runner::run(test_spec);
        }
        TestMode::Full => {
            let bench_results = benchmark_runner::run_comparison(test_spec);
            match_runner::run(test_spec);
            let _benchmark_analysis = benchmark_analyzer::analyze_comparison_results(bench_results);
        }
    }
}
