use crate::UciMove;

pub fn uciok() -> String {
    String::from("uciok")
}

pub fn readyok() -> String {
    String::from("readyok")
}

pub fn id_name(name: &str) -> String {
    format!("id name {}", name)
}

pub fn id_author(author: &str) -> String {
    format!("id author {}", author)
}

pub fn bestmove(mv: UciMove) -> String {
    match mv.promoted_to {
        Some(piece) => format!("{}{}{}", mv.origin, mv.destination, piece),
        None => format!("bestmove {}{}", mv.origin, mv.destination),
    }
}
