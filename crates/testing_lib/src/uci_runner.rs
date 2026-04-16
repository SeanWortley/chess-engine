use crate::EngineConfig;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    time::SystemTime,
};

pub struct UciSession {
    process: Child,
    input: BufWriter<ChildStdin>,
    output: BufReader<ChildStdout>,
}

impl UciSession {
    fn from_child(mut child: Child) -> Self {
        let stdin = child.stdin.take().expect("Can't take stdin");
        let stdout = child.stdout.take().expect("Can't take stdout");

        Self {
            process: child,
            input: BufWriter::new(stdin),
            output: BufReader::new(stdout),
        }
    }

    pub fn send_line(&mut self, line: &str) {
        writeln!(self.input, "{}", line).expect("Failed to write engine stdin");
        self.input.flush().expect("Failed to flush engine stdin");
    }

    pub fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.output
            .read_line(&mut line)
            .expect("Failed to read engine stdout");
        line.trim().to_string()
    }

    pub fn shutdown(mut self) {
        self.send_line("quit");
        let _ = self.process.wait();
    }

    fn read_until(&mut self, starts_with: &str) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

        loop {
            let line = self.read_line();
            lines.push(line);
            if lines[lines.len() - 1].starts_with(starts_with) {
                return lines;
            }
        }
    }

    fn hand_shake(&mut self) {
        self.send_line("uci");
        self.read_until("uciok");

        self.send_line("isready");
        self.read_until("readyok");
    }

    fn set_position(&mut self, fen: &str) {
        self.send_line(format!("position fen {}", fen).as_str());
    }
}

pub fn run_perft_case(engine_config: &EngineConfig, fen: &str, depth: u8) -> (u64, u64) {
    let child = launch_child_from_config(engine_config);
    let mut session = UciSession::from_child(child);

    session.hand_shake();
    session.set_position(fen);

    let start_time = SystemTime::now();
    session.send_line(format!("go perft {}", depth).as_str());
    let lines = session.read_until("perft");
    let end_time = SystemTime::now();

    session.shutdown();

    let line = lines.get(lines.len() - 1).expect("Failed to get last line");
    let nodes: u64 = line
        .rsplit(" ")
        .next()
        .expect("Split into none")
        .parse()
        .expect("Nodes num is not a valid number");

    let duration = end_time.duration_since(start_time).unwrap();
    let time_ms = duration.as_millis();

    (nodes, time_ms as u64)
}

fn launch_child_from_config(engine: &EngineConfig) -> Child {
    let mut command = match &engine.docker_path {
        // Docker support can be implemented later hopefully :)
        Some(docker_path) => Command::new(docker_path),
        None => Command::new(&engine.binary_path),
    };

    command
        .args(&engine.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn engine process")
}
