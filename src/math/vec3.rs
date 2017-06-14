use std::ops::{Mul, Div, Add, Sub};

/// Stores a 3 dimensional coordinate set
///
/// Note that it is used as a point, as a direction, and as general 3 item
/// storage.
#[derive(Copy,Clone,PartialEq)]
pub struct Vec3<T> {
    /// x (index 0) element
    pub x: T,
    /// y (index 1) element
    pub y: T,
    /// z (index 2) element
    pub z: T
}

/// Construct methods for Vec3<f32>
impl Vec3<f32> {
    /// Constructs a new Vec3<f32> with x, y and z values set to 0
    #[inline]
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
    /// Constructs a new Vec3<f32> with x, y and z values set to 0
    ///
    /// Same as [zero](#method.zero), provided for ease of use
    #[inline]
    pub fn new() -> Self {
        Self::zero()
    }
    /// Constructs a new Vec3<f32> with x, y and z values set to 1
    #[inline]
    pub fn one() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }
    /// Constructs a new unit-length Vec3<f32> pointing toward negative X (left)
    #[inline]
    pub fn left() -> Self {
        Vec3 {
            x: -1.0,
            y:  0.0,
            z:  0.0
        }
    }
    /// Constructs a new unit-length Vec3<f32> pointing toward positive X
    /// (right)
    #[inline]
    pub fn right() -> Self {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    }
    /// Constructs a new unit-length Vec3<f32> pointing toward negative Y (down)
    #[inline]
    pub fn down() -> Self {
        Vec3 {
            x:  0.0,
            y: -1.0,
            z:  0.0
        }
    }
    /// Constructs a new unit-length Vec3<f32> pointing toward positive Y
    /// (right)
    #[inline]
    pub fn up() -> Self {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    }
    /// Constructs a new unit-length Vec3<f32> pointing toward negative Z
    /// (backward)
    #[inline]
    pub fn backward() -> Self {
        Vec3 {
            x:  0.0,
            y:  0.0,
            z: -1.0
        }
    }
    /// Constructs a new unit-length Vec3<f32> pointing toward positive Z
    /// (forward)
    #[inline]
    pub fn forward() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }
    /// Constructs a new Vec3<f32> from values x, y and z
    #[inline]
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }
}

/// Get methods for Vec3<f32>
impl Vec3<f32> {
    /// "Swizzle-like" method returning the x value
    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }
    /// "Swizzle-like" method returning the y value
    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }
    /// "Swizzle-like" method returning the z value
    pub fn z(&self) -> f32 {
        self.z
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to x and
    /// `y` set to x
    #[inline]
    pub fn xx(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.x,
            y: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to x and
    /// `y` set to y
    #[inline]
    pub fn xy(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.x,
            y: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to x and
    /// `y` set to z
    #[inline]
    pub fn xz(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.x,
            y: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to y and
    /// `y` set to x
    #[inline]
    pub fn yx(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.y,
            y: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to y and
    /// `y` set to y
    #[inline]
    pub fn yy(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.y,
            y: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to y and
    /// `y` set to z
    #[inline]
    pub fn yz(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.y,
            y: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to z and
    /// `y` set to x
    #[inline]
    pub fn zx(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.z,
            y: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to z and
    /// `y` set to y
    #[inline]
    pub fn zy(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.z,
            y: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec2<f32> with `x` set to z and
    /// `y` set to z
    #[inline]
    pub fn zz(&self) -> super::vec2::Vec2<f32> {
        super::vec2::Vec2 {
            x: self.z,
            y: self.z
        }
    }

    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to x and `z` set to x
    #[inline]
    pub fn xxx(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to x and `z` set to y
    #[inline]
    pub fn xxy(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to x and `z` set to z
    #[inline]
    pub fn xxz(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to y and `z` set to x
    #[inline]
    pub fn xyx(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to y and `z` set to y
    #[inline]
    pub fn xyy(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to y and `z` set to z
    #[inline]
    pub fn xyz(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to z and `z` set to x
    #[inline]
    pub fn xzx(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to z and `z` set to y
    #[inline]
    pub fn xzy(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to z and `z` set to z
    #[inline]
    pub fn xzz(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to x and `z` set to x
    #[inline]
    pub fn yxx(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to x and `z` set to y
    #[inline]
    pub fn yxy(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to x and `z` set to z
    #[inline]
    pub fn yxz(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to y and `z` set to x
    #[inline]
    pub fn yyx(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to y and `z` set to y
    #[inline]
    pub fn yyy(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to y and `z` set to z
    #[inline]
    pub fn yyz(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to z and `z` set to x
    #[inline]
    pub fn yzx(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to z and `z` set to y
    #[inline]
    pub fn yzy(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to z and `z` set to z
    #[inline]
    pub fn yzz(&self) -> Self {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.z
        }
    }

    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to x and `z` set to x
    #[inline]
    pub fn zxx(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to x and `z` set to y
    #[inline]
    pub fn zxy(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to x and `z` set to z
    #[inline]
    pub fn zxz(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to y and `z` set to x
    #[inline]
    pub fn zyx(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to y and `z` set to y
    #[inline]
    pub fn zyy(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to y and `z` set to z
    #[inline]
    pub fn zyz(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to z and `z` set to x
    #[inline]
    pub fn zzx(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to z and `z` set to y
    #[inline]
    pub fn zzy(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to z and `z` set to z
    #[inline]
    pub fn zzz(&self) -> Self {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.z
        }
    }

    /// Returns this Vec3<f32> as a 3 element array
    #[inline]
    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// Calc methods for Vec3<f32>
impl Vec3<f32> {
    /// Calculates the dot product of two Vec3<f32>
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
    /// Calculates the cross product of two Vec3<f32>
    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }
    /// Calculates the square-length of this Vec3<f32>
    ///
    /// This can be used for faster comparisons of lengths of vectors, but is
    /// not a replacement for [len](#method.len).
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.dot(self)
    }
    /// Calculates the length of this Vec3<f32>
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }
    /// Calculates the square-distance between two Vec3<f32>
    ///
    /// This can be used for faster comparisons of distances, but is not a
    /// replacement for [dist](#method.dist)
    #[inline]
    pub fn sqr_dist(&self, b: &Self) -> f32 {
        (*self - *b).sqr_len()
    }
    /// Calculates the distance between two Vec3<f32>
    #[inline]
    pub fn dist(&self, b: &Self) -> f32 {
        (*self - *b).len()
    }
    /// Calculates and returns a normalized copy of this Vec3<f32>
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.len();

        Vec3 {
            x: self.x/len,
            y: self.y/len,
            z: self.z/len
        }
    }
}

/// Mutate methods for Vec3<f32>
impl Vec3<f32> {
    /// Sets a mutable Vec3<f32> to values x, y and z
    #[inline]
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    /// Sets a mutable Vec3<f32> to values in a three element f32 array
    #[inline]
    pub fn set_array(&mut self, a: [f32; 3]) {
        self.x = a[0];
        self.y = a[1];
        self.z = a[2];
    }
}

impl Mul for Vec3<f32> {
    type Output = Vec3<f32>;
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x*rhs.x,
            y: self.y*rhs.y,
            z: self.z*rhs.z
        }
    }
}
impl Div for Vec3<f32> {
    type Output = Vec3<f32>;
    fn div(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x/rhs.x,
            y: self.y/rhs.y,
            z: self.z/rhs.z
        }
    }
}
impl Add for Vec3<f32> {
    type Output = Vec3<f32>;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl Sub for Vec3<f32> {
    type Output = Vec3<f32>;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dot_test() {
    }
}
