use std::time::{Duration, Instant};

pub struct Throttle {
    min_interval: Duration,
    last_run: Option<Instant>,
}

impl Throttle {
    /// Throttles to max one run per interval `min_interval`.
    pub fn one_run_per(min_interval: Duration) -> Self {
        Throttle {
            min_interval,
            last_run: None,
        }
    }

    /// Throttles to max `runs_per_sec` runs per second.
    pub fn max_runs_per_sec(runs_per_sec: u64) -> Self {
        Throttle::one_run_per(Duration::from_millis(1000 / runs_per_sec))
    }

    pub fn throttle<F>(&mut self, f: F)
    where
        F: FnOnce(),
    {
        if self
            .last_run
            .map(|instant| instant.elapsed() < self.min_interval)
            .unwrap_or(false)
        {
            return;
        }

        f();
        self.last_run = Some(Instant::now());
    }
}
