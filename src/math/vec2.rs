//! 2 dimensional vector mathematics

/// Stores a 2 dimensional set
///
/// Used for points, vertices, directions, velocities, etc.
pub struct Vec2<T> {
    /// X (index 0) coordinate
    pub x: T,
    /// Y (index 1) coordinate
    pub y: T
}

/// Construct methods for Vec2<f32>
impl Vec2<f32> {
    /// Constructs a zeroed Vec2<f32>
    fn zero() -> Self {
        Vec2 {
            x: 0.0f32,
            y: 0.0f32
        }
    }
}
