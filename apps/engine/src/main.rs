use engine_lib::prelude::*;
mod uci_loop;

fn main() {
    let engine = DefaultEngine::default();
    uci_loop::run(engine)
}
