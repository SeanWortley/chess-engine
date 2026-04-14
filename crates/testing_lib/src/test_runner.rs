use crate::{
    match_runner,
    test_spec::{TestMode, TestSpec},
};

pub fn run(test_spec: &TestSpec) {
    match test_spec.mode() {
        TestMode::BenchmarkSolo => {
            // TODO
        }
        TestMode::BenchmarkComparison => {
            // TODO
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
