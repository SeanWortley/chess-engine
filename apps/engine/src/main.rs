use engine_lib::{engine::V4Engine, prelude::*};
mod uci_loop;
mod uci_reporter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let engine_kind = args
        .iter()
        .find(|a| a.starts_with("--engine="))
        .map(|a| a.trim_start_matches("--engine="))
        .unwrap_or("v4");

    match engine_kind {
        "v1" | "random" => {
            uci_loop::run(V1Engine::v1());
        }
        "v2" | "purenegamax" => {
            uci_loop::run(V2Engine::v2());
        }
        "v3" | "deepening" => {
            uci_loop::run(V3Engine::v3());
        }
        "v4" | "ab" => {
            uci_loop::run(V4Engine::v4());
        }
        _ => {
            panic!("Preset not found");
        }
    }
}
