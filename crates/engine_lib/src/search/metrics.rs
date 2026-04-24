pub struct SearchMetrics {
    pub nodes: u64,
}

impl SearchMetrics {
    pub fn new() -> Self {
        Self { nodes: 0 }
    }

    pub fn increment(&mut self) {
        self.nodes += 1;
    }

    pub fn total(&self) -> u64 {
        self.nodes
    }

    pub fn reset(&mut self) {
        self.nodes = 0;
    }
}
