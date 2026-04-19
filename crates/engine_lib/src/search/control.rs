use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
pub struct SearchConstraint {
    pub max_depth: Option<u8>,
    pub movetime: Option<Duration>,
}

impl SearchConstraint {
    pub fn fixed_depth(depth: u8) -> Self {
        SearchConstraint {
            max_depth: Some(depth),
            movetime: None,
        }
    }

    pub fn movetime(movetime: Duration) -> Self {
        SearchConstraint {
            max_depth: None,
            movetime: Some(movetime),
        }
    }
}

#[derive(Clone)]
pub struct SearchControl {
    should_stop: Arc<AtomicBool>,
    start_time: Instant,
    deadline: Option<Instant>,
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
        }
    }

    pub fn should_stop(&self) -> bool {
        if self.should_stop.load(Ordering::Relaxed) == true {
            return true;
        }
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
