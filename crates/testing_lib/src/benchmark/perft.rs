use crate::{BenchmarkResult, EngineConfig, benchmark::positions, uci_runner};

const TOTAL_CASES: u8 = 11;

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
    for i in 1..5 {
        let case_no = i;

        let result = uci_runner::run_perft_case(engine, positions::STARTPOS, i);
        nodes_time_sum.0 += result.0;
        nodes_time_sum.1 += result.1;

        println!(
            "[perft] DONE  engine={} pos=startpos depth={} nodes={} ms={} ({}/{})",
            engine.name, case_no, result.0, result.1, case_no, TOTAL_CASES
        );
    }
    nodes_time_sum
}

fn kiwipete(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes_time_sum: (u64, u64) = (0, 0);
    for i in 1..=4 {
        let case_no = 4 + i;

        let result = uci_runner::run_perft_case(engine, positions::KIWIPETE, i);
        nodes_time_sum.0 += result.0;
        nodes_time_sum.1 += result.1;

        println!(
            "[perft] DONE  engine={} pos=kiwipete depth={} nodes={} ms={} ({}/{})",
            engine.name, i, result.0, result.1, case_no, TOTAL_CASES
        );
    }
    nodes_time_sum
}

fn talkchess_bug_finder(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes_time_sum: (u64, u64) = (0, 0);
    for i in 1..=4 {
        let case_no = 7 + i;

        let result = uci_runner::run_perft_case(engine, positions::TALKCHESS_BUG_FINDER, i);
        nodes_time_sum.0 += result.0;
        nodes_time_sum.1 += result.1;

        println!(
            "[perft] DONE  engine={} pos=talkchess_bug_finder depth={} nodes={} ms={} ({}/{})",
            engine.name, i, result.0, result.1, case_no, TOTAL_CASES
        );
    }
    nodes_time_sum
}
