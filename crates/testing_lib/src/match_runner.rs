use std::path::PathBuf;

use crate::{results::MatchResults, test_spec::TestSpec};

pub fn run(test_spec: &TestSpec) -> MatchResults {
    let _baseline = &test_spec.baseline;
    let _candidate = &test_spec.baseline;
    let _config = &test_spec.match_config;
    MatchResults {
        draws: 1,
        losses: 1,
        wins: 1,
    }
}

fn _cutechess_path() -> PathBuf {
    std::env::var("CUTECHESS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("cutechess-cli"))
}
