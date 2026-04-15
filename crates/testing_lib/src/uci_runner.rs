use crate::{BenchmarkResult, EngineConfig};
use std::process::{Child, Command, Stdio};

pub fn run_perft_speed(engine_config: &EngineConfig, depth: u8) -> BenchmarkResult {
    let engine = spawn_engine(engine_config);
}

fn spawn_engine(engine: &EngineConfig) -> Child {
    Command::new(&engine.binary_path)
        .args(&engine.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start engine")
}
