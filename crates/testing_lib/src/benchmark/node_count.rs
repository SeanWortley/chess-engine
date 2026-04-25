use crate::{BenchmarkResult, EngineConfig, benchmark::positions, uci_runner};

pub fn node_count(engine: &EngineConfig) -> BenchmarkResult {
    let mut nodes = 0;

    nodes += kiwipete(engine);
    nodes += talkchess_bug_finder(engine);

    BenchmarkResult::NodesEvaluated { nodes }
}

fn kiwipete(engine: &EngineConfig) -> u64 {
    let mut nodes: u64 = 0;

    for i in 1..=5 {
        let result = uci_runner::run_node_count_case(engine, positions::KIWIPETE, i);
        nodes += result;

        println!(
            "[node_count] DONE  engine={} pos=kiwipete depth={} nodes={}",
            engine.name, i, result
        );
    }
    nodes
}

fn talkchess_bug_finder(engine: &EngineConfig) -> u64 {
    let mut nodes: u64 = 0;

    for i in 1..=5 {
        let result = uci_runner::run_node_count_case(engine, positions::TALKCHESS_BUG_FINDER, i);
        nodes += result;

        println!(
            "[node_count] DONE  engine={} pos=talkchess_bug_finder depth={} nodes={}",
            engine.name, i, result
        );
    }
    nodes
}
