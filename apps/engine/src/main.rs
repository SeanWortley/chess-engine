use engine_lib::{
    engine::{V4Engine, V5Engine, V6Engine, V7Engine, V8Engine},
    prelude::*,
};
mod uci_loop;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let engine_kind = args
        .iter()
        .find(|a| a.starts_with("--engine="))
        .map(|a| a.trim_start_matches("--engine="))
        .unwrap_or("v8");

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
        "v5" | "pesto" => {
            uci_loop::run(V5Engine::v5());
        }
        "v6" | "mvv_lva" => {
            uci_loop::run(V6Engine::v6());
        }
        "v7" | "tt" => {
            uci_loop::run(V7Engine::v7());
        }
        "v8" | "quiescent" => {
            uci_loop::run(V8Engine::v8());
        }
        _ => {
            panic!("Preset not found");
        }
    }
}
