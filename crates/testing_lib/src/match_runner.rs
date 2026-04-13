use crate::{
    engine::EngineConfig,
    match_config::{Constraint, MatchConfig, OpeningSource},
    results::MatchResults,
    test_spec::TestSpec,
};

pub fn run(test_spec: &TestSpec) -> MatchResults {
    let baseline = &test_spec.baseline;
    let candidate = &test_spec.baseline;
    let config = &test_spec.match_config;
    let args = build_args(&baseline, &candidate, &config.as_ref().unwrap());
    MatchResults {
        wins: 1,
        draws: 1,
        losses: 1,
    }
}

fn build_args(
    baseline: &EngineConfig,
    candidate: &EngineConfig,
    config: &MatchConfig,
) -> Vec<String> {
    let mut args = Vec::new();

    // Baseline arg
    let mut baseline_arg = String::new();
    baseline_arg.push_str("-engine ");
    baseline_arg.push_str(
        format!(
            "name={} cmd={} ",
            baseline.name,
            baseline.binary.to_str().unwrap()
        )
        .as_str(),
    );
    for (i, arg) in baseline.args.iter().enumerate() {
        baseline_arg.push_str(format!("arg{}={} ", i + 1, arg).as_str());
    }
    baseline_arg.push_str("proto=uci");
    args.push(baseline_arg);

    // Candidate arg
    let mut candidate_arg = String::new();
    candidate_arg.push_str("-engine ");
    candidate_arg.push_str(
        format!(
            "name={} cmd={} ",
            candidate.name,
            candidate.binary.to_str().unwrap()
        )
        .as_str(),
    );
    for (i, arg) in candidate.args.iter().enumerate() {
        candidate_arg.push_str(format!("arg{}={} ", i + 1, arg).as_str());
    }
    candidate_arg.push_str("proto=uci");
    args.push(candidate_arg);

    // Shared args
    args.push(String::from("-each"));
    args.push(match config.constraint {
        Constraint::FixedDepth(depth) => {
            format!("tc=inf depth={}", depth)
        }
        Constraint::NodeBudget(nodes) => {
            format!("tc=inf nodes={}", nodes)
        }
        Constraint::Standard(base, increment) => {
            format!("tc={:.3}+{:.3}", base, increment)
        }
        Constraint::FixedMoveTime(move_time) => {
            format!("tc=inf movetime={}", move_time)
        }
    });

    // Max rounds (for sprt early stopping)
    args.push(format!("-rounds {}", config.max_rounds));

    // Opening source
    match &config.openings {
        OpeningSource::StartPos => {}
        OpeningSource::Epd(path) => args.push(format!(
            "-openings file={} format=edp order=random plies=8",
            path.display()
        )),
        OpeningSource::Pgn(path) => args.push(format!(
            "-openings file={} format=pgn order=random plies=8",
            path.display()
        )),
    }
    args.push(String::from("-repeat"));

    // Output
    args.push(String::from(format!(
        "-pgnout {}_vs_{}_.pgn",
        baseline.name, candidate.name
    )));

    // Sprt
    let sprt = &config.sprt_config;
    args.push(String::from("-sprt"));
    args.push(format!(
        "-sprt elo0={} elo1={} alpha={} beta={}",
        sprt.elo0, sprt.elo1, sprt.alpha, sprt.beta
    ));

    args
}
