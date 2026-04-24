use crate::{BenchMarkComparison, BenchmarkResult, EngineConfig, benchmark::positions, uci_runner};

const TOTAL_CASES: u8 = 11;

pub fn nodes_evaluated(engine: &EngineConfig) -> BenchmarkResult {
    let _result = startpos(engine);

    BenchmarkResult::NodesEvaluated { nodes: 1 }
}

fn startpos(engine: &EngineConfig) -> u64 {
    let mut nodes: u64 = 0;

    for i in 1..6 {
        let case_no = i;

        let result = uci_runner::run_nodes_evaluated_case(engine, positions::STARTPOS, i);
        nodes += result;
    }
    1
}
