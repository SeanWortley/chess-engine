use crate::{BenchmarkResult, EngineConfig, uci_runner};

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const TALKCHESS_BUG_FINDER: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

pub fn perft_speed_test(engine: &EngineConfig) -> BenchmarkResult {
    let mut nodes_time_sum: (u64, u64) = (0, 0);

    let result = startpos(engine);
    nodes_time_sum.0 += result.0;
    nodes_time_sum.1 += result.1;

    let result = kiwipete(engine);
    nodes_time_sum.0 += result.0;
    nodes_time_sum.1 += result.1;

    let result = talkchess_bug_finder(engine);
    nodes_time_sum.0 += result.0;
    nodes_time_sum.1 += result.1;

    let nps = if nodes_time_sum.1 == 0 {
        0
    } else {
        nodes_time_sum.0.saturating_mul(1000) / nodes_time_sum.1
    };

    BenchmarkResult::PerftSpeed {
        nodes: nodes_time_sum.0,
        duration_ms: nodes_time_sum.1,
        nps,
    }
}

fn startpos(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes_time_sum: (u64, u64) = (0, 0);
    for i in 1..=4 {
        let result = uci_runner::run_perft_case(engine, STARTPOS, i);
        nodes_time_sum.0 += result.0;
        nodes_time_sum.1 += result.1;
    }
    nodes_time_sum
}

fn kiwipete(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes_time_sum: (u64, u64) = (0, 0);
    for i in 1..=3 {
        let result = uci_runner::run_perft_case(engine, KIWIPETE, i);
        nodes_time_sum.0 += result.0;
        nodes_time_sum.1 += result.1;
    }
    nodes_time_sum
}

fn talkchess_bug_finder(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes_time_sum: (u64, u64) = (0, 0);
    for i in 1..=3 {
        let result = uci_runner::run_perft_case(engine, TALKCHESS_BUG_FINDER, i);
        nodes_time_sum.0 += result.0;
        nodes_time_sum.1 += result.1;
    }
    nodes_time_sum
}
