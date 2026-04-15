use crate::{
    benchmark_runner, match_runner,
    test_spec::{TestMode, TestSpec},
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
            // TODO
            match_runner::run(test_spec);
        }
    }
}
