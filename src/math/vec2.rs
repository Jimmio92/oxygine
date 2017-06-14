use std::ops::{Mul, Div, Add, Sub};

/// Vec2 stores a 2 dimensional point, direction, or velocity. It can also just
/// be used to store two of any type, though most of the usefulness is only
/// implemented for use on f32.
#[derive(Copy,Clone,PartialEq)]
pub struct Vec2<T> {
    /// x (index 0) element
    pub x: T,
    /// y (index 1) element
    pub y: T
}

/// Construct methods for generic Vec2
impl<T> Vec2<T> {
    /// Constructs a generic Vec2 from values x and y
    pub fn from(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y
        }
    }
}

/// Construct methods for Vec2<f32>
impl Vec2<f32> {
    /// Constructs a new Vec2<f32> with x and y values set to 0
    #[inline]
    pub fn zero() -> Self {
        Vec2 {
            x: 0.0,
            y: 0.0
        }
    }
    /// Constructs a new Vec2<f32> with x and y values set to 0
    ///
    /// Same as [zero](#method.zero), provided for ease of use
    #[inline]
    pub fn new() -> Self {
        Self::zero()
    }
    /// Constructs a new Vec2<f32> with x and y values set to 1
    #[inline]
    pub fn one() -> Self {
        Vec2 {
            x: 1.0,
            y: 1.0
        }
    }
    /// Constructs a new unit-length Vec2<f32> pointing toward negative X (left)
    #[inline]
    pub fn left() -> Self {
        Vec2 {
            x: -1.0,
            y:  0.0
        }
    }
    /// Constructs a new unit-length Vec2<f32> pointing toward positive X
    /// (right)
    #[inline]
    pub fn right() -> Self {
        Vec2 {
            x: 1.0,
            y: 0.0
        }
    }
    /// Constructs a new unit-length Vec2<f32> pointing toward negative Y (down)
    #[inline]
    pub fn down() -> Self {
        Vec2 {
            x:  0.0,
            y: -1.0
        }
    }
    /// Constructs a new unit-length Vec2<f32> pointing toward positive Y
    /// (right)
    #[inline]
    pub fn up() -> Self {
        Vec2 {
            x: 0.0,
            y: 1.0
        }
    }
}

/// Get methods for Vec2<T> where T implements the Copy trait (this includes
/// built-in types such as f32, u32, etc.)
impl<T> Vec2<T> where T: Copy {
    /// "Swizzle-like" method returning the x value
    #[inline]
    pub fn x(&self) -> T {
        self.x
    }
    /// "Swizzle-like" method returning the y value
    #[inline]
    pub fn y(&self) -> T {
        self.y
    }
    /// "Swizzle-like" method returning a new Vec2 with `x` set to x and
    /// `y` set to x
    #[inline]
    pub fn xx(&self) -> Self {
        Vec2 {
            x: self.x,
            y: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec2 with `x` set to x and
    /// `y` set to y
    #[inline]
    pub fn xy(&self) -> Self {
        Vec2 {
            x: self.x,
            y: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec2 with `x` set to y and
    /// `y` set to x
    #[inline]
    pub fn yx(&self) -> Self {
        Vec2 {
            x: self.y,
            y: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec2 with `x` set to y and
    /// `y` set to y
    #[inline]
    pub fn yy(&self) -> Self {
        Vec2 {
            x: self.y,
            y: self.y
        }
    }
    /// Returns this Vec2 as a 2 element array
    #[inline]
    pub fn as_array(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

/// Calc methods for Vec2<f32>
impl Vec2<f32> {
    /// Calculates the dot product of two Vec2<f32>
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y
    }
    /// Calculates the square-length of this Vec2<f32>
    ///
    /// This can be used for faster comparisons of lengths of vectors, but is
    /// not a replacement for [len](#method.len).
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.dot(self)
    }
    /// Calculates the length of this Vec2<f32>
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }
    /// Calculates the square-distance between two Vec2<f32>
    ///
    /// This can be used for faster comparisons of distances, but is not a
    /// replacement for [dist](#method.dist)
    #[inline]
    pub fn sqr_dist(&self, b: &Self) -> f32 {
        (*self - *b).sqr_len()
    }
    /// Calculates the distance between two Vec2<f32>
    #[inline]
    pub fn dist(&self, b: &Self) -> f32 {
        (*self - *b).len()
    }
    /// Calculates and returns a normalized copy of this Vec2<f32>
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.len();

        Vec2 {
            x: self.x/len,
            y: self.y/len
        }
    }
}

/// Mutate methods for Vec2<T> where T implements the Copy trait (this includes
/// built-in types such as f32, u32, etc.)
impl<T> Vec2<T> where T: Copy {
    /// Sets a mutable Vec2<f32> to values x and y
    #[inline]
    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
    /// Sets a mutable Vec2<f32> to values in a two element array
    #[inline]
    pub fn set_array(&mut self, a: [T; 2]) {
        self.x = a[0];
        self.y = a[1];
    }
}

impl Mul for Vec2<f32> {
    type Output = Vec2<f32>;
    fn mul(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x*rhs.x,
            y: self.y*rhs.y
        }
    }
}
impl Div for Vec2<f32> {
    type Output = Vec2<f32>;
    fn div(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x/rhs.x,
            y: self.y/rhs.y
        }
    }
}
impl Add for Vec2<f32> {
    type Output = Vec2<f32>;
    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl Sub for Vec2<f32> {
    type Output = Vec2<f32>;
    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dot_test() {
        use super::Vec2;
        let guard_fov = 120.0; //degrees
        let guard_pos = Vec2::from(1.0, 3.0);
        let guard_dir = Vec2::up() + Vec2::right();

        let player_pos = Vec2::from(3.0, 2.0);

        // normalize vectors
        let guard_dirn = guard_dir.normalized();
        let guard_to_player = Vec2::normalized(&(player_pos - guard_pos));

        // check angle
        let angle = guard_dirn.dot(&guard_to_player).acos().to_degrees();

        // if this succeeds, the guard can't see us!
        assert!(angle > guard_fov/2.0);
        assert!(angle > 71.0 && angle < 72.0);

    }
}
