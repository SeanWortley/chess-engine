use std::{env, path::PathBuf, process::Command};

use crate::{
    config::{Constraint, EngineConfig, MatchConfig, OpeningSource},
    spec::TestSpec,
};

pub fn run(test_spec: &TestSpec) {
    let baseline = test_spec.baseline();
    let candidate = test_spec.candidate().unwrap_or(baseline);
    let config = test_spec
        .match_config()
        .expect("match_config must be present when running a cutechess match");

    let args = build_args(baseline, candidate, config);

    Command::new(cutechess_path())
        .args(args)
        .spawn()
        .expect("failed to execute cutechess")
        .wait()
        .expect("failed to wait for cutechess");
}

// Only works on linux :(
fn cutechess_path() -> PathBuf {
    env::var("CUTECHESS_CLI")
        .map(PathBuf::from)
        .expect("Unable to find cutechess path")
}

fn build_args(
    baseline: &EngineConfig,
    candidate: &EngineConfig,
    config: &MatchConfig,
) -> Vec<String> {
    let mut args = Vec::new();

    // Candidate arg
    args.push(String::from("-engine"));
    args.push(format!("name={}", candidate.name));
    args.push(format!("cmd={}", candidate.binary_path.to_str().unwrap()));
    for arg in candidate.args.iter() {
        args.push(format!("arg={}", arg));
    }
    args.push(String::from("proto=uci"));

    // Baseline arg
    args.push(String::from("-engine"));
    args.push(format!("name={}", baseline.name));
    args.push(format!("cmd={}", baseline.binary_path.to_str().unwrap()));
    for arg in baseline.args.iter() {
        args.push(format!("arg={}", arg));
    }
    args.push(String::from("proto=uci"));

    // Shared args
    args.push(String::from("-each"));
    args.push(match config.constraint {
        Constraint::FixedDepth(_depth) => {
            format!("tc=inf")
        }
        Constraint::NodeBudget(_nodes) => {
            format!("tc=inf")
        }
        Constraint::Standard(base, increment) => {
            format!("tc={:.3}+{:.3}", base, increment)
        }
        Constraint::FixedMoveTime(_move_time) => {
            format!("tc=inf")
        }
    });
    match config.constraint {
        Constraint::FixedDepth(depth) => args.push(format!("depth={}", depth)),
        Constraint::NodeBudget(nodes) => args.push(format!("nodes={}", nodes)),
        Constraint::Standard(_, _) => {}
        Constraint::FixedMoveTime(move_time) => args.push(format!("movetime={}", move_time)),
    }

    // Max rounds (for sprt early stopping)
    args.push(String::from("-rounds"));
    args.push(config.max_rounds.to_string());

    // Opening source
    match &config.openings {
        OpeningSource::StartPos => {}
        OpeningSource::Epd(path) => {
            args.push(String::from("-openings"));
            args.push(format!("file={}", path.display()));
            args.push(String::from("format=epd"));
            args.push(String::from("order=random"));
            args.push(String::from("plies=8"));
        }
        OpeningSource::Pgn(path) => {
            args.push(String::from("-openings"));
            args.push(format!("file={}", path.display()));
            args.push(String::from("format=pgn"));
            args.push(String::from("order=random"));
            args.push(String::from("plies=8"));
        }
    }
    args.push(String::from("-repeat"));

    // Output
    args.push(String::from("-pgnout"));
    args.push(format!("{}_vs_{}_.pgn", baseline.name, candidate.name));

    // Sprt
    args.push(String::from("-sprt"));
    let sprt = &config.sprt_config;
    args.push(format!("elo0={}", sprt.elo0));
    args.push(format!("elo1={}", sprt.elo1));
    args.push(format!("alpha={}", sprt.alpha));
    args.push(format!("beta={}", sprt.beta));

    args
}
