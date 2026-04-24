use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use crate::search::metrics::SearchMetrics;

#[derive(Clone, Copy)]
pub struct SearchConstraint {
    pub max_depth: Option<u8>,
    pub movetime: Option<Duration>,
    pub node_budget: Option<u64>,
}

impl SearchConstraint {
    pub fn fixed_depth(depth: u8) -> Self {
        SearchConstraint {
            max_depth: Some(depth),
            movetime: None,
            node_budget: None,
        }
    }

    pub fn movetime(movetime: Duration) -> Self {
        SearchConstraint {
            max_depth: None,
            movetime: Some(movetime),
            node_budget: None,
        }
    }

    pub fn node_budget(node_budget: u64) -> Self {
        SearchConstraint {
            max_depth: None,
            movetime: None,
            node_budget: Some(node_budget),
        }
    }
}

#[derive(Clone)]
pub struct SearchControl {
    should_stop: Arc<AtomicBool>,
    start_time: Instant,
    deadline: Option<Instant>,
    node_budget: Option<u64>,
}

impl SearchControl {
    pub fn new(constraint: SearchConstraint) -> Self {
        let start_time = Instant::now();
        let deadline = constraint
            .movetime
            .map(|t| start_time + std::time::Duration::from_secs_f64(t.as_secs_f64() * 0.9));
        SearchControl {
            should_stop: Arc::new(AtomicBool::new(false)),
            start_time,
            deadline,
            node_budget: constraint.node_budget,
        }
    }

    pub fn should_stop(&self, metrics: &SearchMetrics) -> bool {
        // First check flag
        if self.should_stop.load(Ordering::Relaxed) {
            return true;
        }

        // Then check nodes
        if let Some(budget) = self.node_budget {
            if metrics.total() >= budget {
                return true;
            }
        }

        // Then check deadline
        match self.deadline {
            Some(deadline) => {
                return Instant::now() >= deadline;
            }
            None => {
                return false;
            }
        }
    }

    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}
