use crate::{
    results::MatchResults,
    test_spec::{self, TestSpec},
};

pub fn run(test_spec: &TestSpec) -> MatchResults {
    let baseline = test_spec.baseline;
    let candidate = test_spec.baseline;
    let config = test_spec.match_config;
}
