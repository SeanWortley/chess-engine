use crate::{BenchmarkResult, EngineConfig};
use std::process::{Child, Command};

pub fn run_perft_speed(engine: &EngineConfig, depth: u8) -> BenchmarkResult {
    start_engine(engine);
}

fn spawn_engine(engine: &EngineConfig) -> Child {
    Command::new(&engine.binary_path).args(&engine.args).spawn();
}
