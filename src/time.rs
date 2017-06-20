//! Time module

use std::time;

/// Provides elapsed time and delta (change-in) time between updates
///
/// The time structure stores three Instants (when the Time structure is
/// created, the previous frame time, and the most recent frame time) and
/// provides the [`delta`](#method.delta) method to calculate the time since
/// the previous update.
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
    /// Updates the Time structure
    /// # Notes
    /// Make sure to call this often (normally once per main loop) to ensure the
    /// times are accurate.
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
    /// # Notes
    /// This does **not** return the actual current time. Instead, it returns
    /// the time it took from the creation of this [`Time`](struct.Time.html)
    /// structure to the previous update. This keeps your actual time one update
    /// behind of *actual*.
    pub fn elapsed(&self) -> f64 {
        let dur = self.now.duration_since(self.start);
        (dur.as_secs() as f64)
        + ((dur.subsec_nanos() as f64) / 1_000_000_000.0)
    }
    /// Calculates the time since the prior update of this Time structure as an
    /// f64
    pub fn delta(&self) -> f64 {
        (self.delta.as_secs() as f64)
        + ((self.delta.subsec_nanos() as f64) / 1_000_000_000.0)
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time as stdtime;

    #[test]
    fn construct() {
        #[allow(unused_variables)]
        let time = ::Time::new();
    }
    #[test]
    fn elapsed() {
        let mut time = ::Time::new();

        let a = time.elapsed();

        thread::sleep(stdtime::Duration::new(0, 5000));

        time.update();

        let b = time.elapsed();

        assert!(a < b);
    }
    #[test]
    fn delta() {
        let mut time = ::Time::new();

        time.update();

        thread::sleep(stdtime::Duration::new(0, 5000));

        time.update();

        assert!(time.delta() >= 5000.0/1_000_000_000.0);
    }
}
