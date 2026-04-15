use crate::EngineConfig;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
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
}

pub fn run_perft_case(engine_config: &EngineConfig, _fen: &str, _depth: u8) -> (u64, u64) {
    let child = launch_child_from_config(engine_config);
    let session = UciSession::from_child(child);
    session.shutdown();

    (0, 0)
}

fn launch_child_from_config(engine: &EngineConfig) -> Child {
    let mut command = match &engine.docker_path {
        // Docker support can be implemented later by mapping this path to
        // a docker/podman executable wrapper command.
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
