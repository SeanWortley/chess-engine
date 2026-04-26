use crate::{
    benchmark::{analysis as benchmark_analyzer, runner as benchmark_runner},
    match_runner, report,
    spec::{TestMode, TestSpec},
};

pub fn run(test_spec: &TestSpec) {
    match test_spec.mode() {
        TestMode::BenchmarkSolo => {
            let _result = benchmark_runner::run_solo(test_spec);
        }
        TestMode::BenchmarkComparison => {
            let results = benchmark_runner::run_comparison(test_spec);
            let analysis = benchmark_analyzer::analyze_comparison_results(results);
            report::print_report(analysis);
        }
        TestMode::Match => {
            match_runner::run(test_spec);
        }
        TestMode::Full => {
            let bench_results = benchmark_runner::run_comparison(test_spec);
            let benchmark_analysis = benchmark_analyzer::analyze_comparison_results(bench_results);
            report::print_report(benchmark_analysis);
            match_runner::run(test_spec);
        }
    }
}
