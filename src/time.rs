//! Time module

use std::time;

/// Time structure
pub struct Time {
    start: time::Instant,
    last: time::Instant,
    now: time::Instant,
    delta: time::Duration
}
/// Construct methods for Time
impl Time {
    /// Constructs a new Time structure, initializing its members to the current
    /// time
    pub fn new() -> Self {
        Time {
            start: time::Instant::now(),
            last: time::Instant::now(),
            now: time::Instant::now(),
            delta: time::Duration::new(0,0)
        }
    }
}
/// Mutate methods for Time
impl Time {
    /// Updates the Time structure. Call this once per main loop if you want to
    /// be able to use it.
    pub fn update(&mut self) {
        self.last = self.now;
        self.now = time::Instant::now();
        self.delta = self.now.duration_since(self.last);
    }
}
/// Calc methods for Time
impl Time {
    /// Calculates the elapsed time since creation of this Time structure as an
    /// f64
    pub fn elapsed(&self) -> f64 {
        let dur = self.now.duration_since(self.start);
        (dur.as_secs() as f64)
        + ((dur.subsec_nanos() as f64) / 1_000_000_000_000.0)
    }
    /// Calculates the time since the last update of this Time structure as an
    /// f64
    pub fn delta(&self) -> f64 {
        (self.delta.as_secs() as f64)
        + ((self.delta.subsec_nanos() as f64) / 1_000_000_000_000.0)
    }
}

#[cfg(test)]
mod time_tests {
    #[test]
    fn construct() {
        #[allow(unused_variables)]
        let time = ::Time::new();
    }
    #[test]
    fn elapsed() {
        let mut time = ::Time::new();

        let a = time.elapsed();
        time.update();
        let b = time.elapsed();

        assert!(a < b);
    }
}
