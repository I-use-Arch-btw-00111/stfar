use std::time::Duration;
use minstant::Instant;

/// A simple timer that accumulates total elapsed time across multiple start/stop cycles.
///
/// - `total` stores the accumulated duration from completed runs.
/// - `started_at` contains Instant while running, or `None` when stopped.
#[derive(Debug, Default, Clone)]
pub struct Timer {
    total: Duration,
    started_at: Option<Instant>,
}

impl Timer {
    /// Create a new timer (stopped, zeroed).
    pub fn new() -> Self {
        Self {
            total: Duration::ZERO,
            started_at: None,
        }
    }

    /// Start the timer. If already running, this is a no-op.
    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
    }

    /// Stop the timer and add the running interval to the total.
    /// If not running, this is a no-op.
    pub fn stop(&mut self) {
        if let Some(since) = self.started_at.take() {
            let elapsed = since.elapsed();
            self.total = self.total.saturating_add(elapsed);
        }
    }

    /// Reset the timer to zero and stop it.
    pub fn reset(&mut self) {
        self.total = Duration::ZERO;
        self.started_at = None;
    }

    /// Returns true if the timer is currently running.
    pub fn is_running(&self) -> bool {
        self.started_at.is_some()
    }

    /// Returns the total accumulated duration, including the current running interval (if any).
    pub fn total_elapsed(&self) -> Duration {
        match self.started_at {
            Some(s) => self.total.saturating_add(s.elapsed()),
            None => self.total,
        }
    }

    /// Convenience: returns total elapsed as seconds (floating-point).
    pub fn total_seconds(&self) -> f64 {
        let d = self.total_elapsed();
        d.as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn total_counter_accumulates() {
        let mut t = Timer::new();
        assert!(!t.is_running());
        t.start();
        sleep(std::time::Duration::from_millis(20));
        t.stop();
        let first = t.total_elapsed();
        assert!(first >= Duration::from_millis(20));

        t.start();
        sleep(std::time::Duration::from_millis(15));
        // leave running and check total includes running interval
        let mid = t.total_elapsed();
        assert!(mid >= first);

        t.stop();
        let second = t.total_elapsed();
        assert!(second >= first);
    }
}
