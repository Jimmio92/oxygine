#![deny(missing_docs)]
#![allow(dead_code)]

//! Oxygine Game Engine

pub mod math;

pub mod time;
use time::Time;

struct Engine {
    time: Time
}
/// Construct methods for Engine
impl Engine {
    /// Constructs a new Engine
    pub fn new() -> Self {
        Engine {
            time: Time::new()
        }
    }
}
/// Mutate methods for Engine
impl Engine {
    /// Updates the Engine structure; call this once per main loop for correct
    /// functionality!
    pub fn update(&mut self) {
        self.time.update();
    }
}

#[cfg(test)]
mod engine_tests {
    #[test]
    fn construct() {
        #[allow(unused_variables)]
        let eng = ::Engine::new();
    }
    #[test]
    fn update() {
        let mut eng = ::Engine::new();

        let a = eng.time.elapsed();
        eng.update();
        let b = eng.time.elapsed();

        assert!(a < b);
    }
}
