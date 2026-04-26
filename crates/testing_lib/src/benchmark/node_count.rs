use crate::{BenchmarkResult, EngineConfig, benchmark::positions, uci_runner};

pub fn node_count(engine: &EngineConfig) -> BenchmarkResult {
    let mut nodes = 0;
    let mut time = 0;

    let result = kiwipete(engine);
    nodes += result.0;
    time += result.1;

    let result = talkchess_bug_finder(engine);
    nodes += result.0;
    time += result.1;

    BenchmarkResult::NodesEvaluated {
        nodes,
        time_ms: time,
    }
}

fn kiwipete(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes: u64 = 0;
    let mut time: u64 = 0;

    for i in 1..=5 {
        let result = uci_runner::run_node_count_case(engine, positions::KIWIPETE, i);
        nodes += result.0;
        time += result.1;

        println!(
            "[node_count] DONE  engine={} pos=kiwipete depth={} nodes={} ms={}",
            engine.name, i, result.0, result.1
        );
    }
    (nodes, time)
}

fn talkchess_bug_finder(engine: &EngineConfig) -> (u64, u64) {
    let mut nodes: u64 = 0;
    let mut time: u64 = 0;

    for i in 1..=5 {
        let result = uci_runner::run_node_count_case(engine, positions::TALKCHESS_BUG_FINDER, i);
        nodes += result.0;
        time += result.1;

        println!(
            "[node_count] DONE  engine={} pos=kiwipete depth={} nodes={} ms={}",
            engine.name, i, result.0, result.1
        );
    }
    (nodes, time)
}
