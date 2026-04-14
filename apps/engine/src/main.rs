use engine_lib::prelude::*;
mod uci_loop;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let engine_kind = args
        .iter()
        .find(|a| a.starts_with("--engine="))
        .map(|a| a.trim_start_matches("--engine="))
        .unwrap_or("best");

    match engine_kind {
        "random" => {
            eprintln!("Initialising random engine");
            uci_loop::run(RandomEngine::random());
        }
        "best" => {
            eprintln!("Initialising best engine");
            uci_loop::run(BestEngine::best());
        }
        _ => {
            panic!("Preset not found");
        }
    }
}
