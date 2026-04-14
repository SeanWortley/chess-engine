use engine_lib::prelude::*;
mod uci_loop;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let engine_kind = args
        .iter()
        .find(|a| a.starts_with("--engine"))
        .map(|a| a.trim_start_matches("--engine="))
        .unwrap();

    match engine_kind {
        "random" => {
            println!("Initialising random engine");
            uci_loop::run(RandomEngine::random());
        }
        "best" => {
            println!("Initialising best engine");
            uci_loop::run(BestEngine::best());
        }
        _ => {
            panic!("Preset not found");
        }
    }
    let engine = BestEngine::best();
    uci_loop::run(engine)
}
