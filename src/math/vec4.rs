use std::ops::{Mul, Div, Add, Sub};

/// Stores a 4 dimensional coordinate set
///
/// Note that it is used as a point, as a direction, and as general 4 item
/// storage.
#[derive(Copy,Clone,PartialEq)]
pub struct Vec4<T> {
    /// x (index 0) element
    pub x: T,
    /// y (index 1) element
    pub y: T,
    /// z (index 2) element
    pub z: T,
    /// w (index 3) element
    pub w: T
}

/// Construct methods for Vec4<f32>
impl Vec4<f32> {
    /// Constructs a new Vec4<f32> with x, y, z and w values set to 0
    #[inline]
    pub fn zero() -> Self {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    }
    /// Constructs a new Vec4<f32> with x, y, z and w values set to 0
    ///
    /// Same as [zero](#method.zero), provided for ease of use
    #[inline]
    pub fn new() -> Self {
        Self::zero()
    }
    /// Constructs a new Vec4<f32> with x, y, z and w values set to 1
    #[inline]
    pub fn one() -> Self {
        Vec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0
        }
    }
    /// Constructs a new Vec4<f32> from values x, y, z and w
    #[inline]
    pub fn from(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
}

/// Get methods for Vec4<f32>
impl Vec4<f32> {
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
    pub fn xxx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.x,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to x and `z` set to y
    #[inline]
    pub fn xxy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.x,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to x and `z` set to z
    #[inline]
    pub fn xxz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.x,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to y and `z` set to x
    #[inline]
    pub fn xyx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.y,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to y and `z` set to y
    #[inline]
    pub fn xyy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.y,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to y and `z` set to z
    #[inline]
    pub fn xyz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to z and `z` set to x
    #[inline]
    pub fn xzx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.z,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to z and `z` set to y
    #[inline]
    pub fn xzy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.z,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to x,
    /// `y` set to z and `z` set to z
    #[inline]
    pub fn xzz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.x,
            y: self.z,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to x and `z` set to x
    #[inline]
    pub fn yxx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.x,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to x and `z` set to y
    #[inline]
    pub fn yxy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.x,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to x and `z` set to z
    #[inline]
    pub fn yxz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.x,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to y and `z` set to x
    #[inline]
    pub fn yyx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.y,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to y and `z` set to y
    #[inline]
    pub fn yyy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.y,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to y and `z` set to z
    #[inline]
    pub fn yyz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.y,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to z and `z` set to x
    #[inline]
    pub fn yzx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.z,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to z and `z` set to y
    #[inline]
    pub fn yzy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.z,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to y,
    /// `y` set to z and `z` set to z
    #[inline]
    pub fn yzz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.y,
            y: self.z,
            z: self.z
        }
    }

    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to x and `z` set to x
    #[inline]
    pub fn zxx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.x,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to x and `z` set to y
    #[inline]
    pub fn zxy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.x,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to x and `z` set to z
    #[inline]
    pub fn zxz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.x,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to y and `z` set to x
    #[inline]
    pub fn zyx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.y,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to y and `z` set to y
    #[inline]
    pub fn zyy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.y,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to y and `z` set to z
    #[inline]
    pub fn zyz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.y,
            z: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to z and `z` set to x
    #[inline]
    pub fn zzx(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.z,
            z: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to z and `z` set to y
    #[inline]
    pub fn zzy(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.z,
            z: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec3<f32> with `x` set to z,
    /// `y` set to z and `z` set to z
    #[inline]
    pub fn zzz(&self) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.z,
            y: self.z,
            z: self.z
        }
    }

    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to x and `w` set to x
    #[inline]
    pub fn xxxx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to x and `w` set to y
    #[inline]
    pub fn xxxy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to x and `w` set to z
    #[inline]
    pub fn xxxz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to x and `w` set to w
    #[inline]
    pub fn xxxw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to y and `w` set to x
    #[inline]
    pub fn xxyx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to y and `w` set to y
    #[inline]
    pub fn xxyy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to y and `w` set to z
    #[inline]
    pub fn xxyz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to y and `w` set to w
    #[inline]
    pub fn xxyw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to z and `w` set to x
    #[inline]
    pub fn xxzx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to z and `w` set to y
    #[inline]
    pub fn xxzy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to z and `w` set to z
    #[inline]
    pub fn xxzz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to z and `w` set to w
    #[inline]
    pub fn xxzw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to w and `w` set to x
    #[inline]
    pub fn xxwx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to w and `w` set to y
    #[inline]
    pub fn xxwy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to w and `w` set to z
    #[inline]
    pub fn xxwz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to x, `z` set to w and `w` set to w
    #[inline]
    pub fn xxww(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.x,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to x and `w` set to x
    #[inline]
    pub fn xyxx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to x and `w` set to y
    #[inline]
    pub fn xyxy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to x and `w` set to z
    #[inline]
    pub fn xyxz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to x and `w` set to w
    #[inline]
    pub fn xyxw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to y and `w` set to x
    #[inline]
    pub fn xyyx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to y and `w` set to y
    #[inline]
    pub fn xyyy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to y and `w` set to z
    #[inline]
    pub fn xyyz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to y and `w` set to w
    #[inline]
    pub fn xyyw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to z and `w` set to x
    #[inline]
    pub fn xyzx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to z and `w` set to y
    #[inline]
    pub fn xyzy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to z and `w` set to z
    #[inline]
    pub fn xyzz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to z and `w` set to w
    #[inline]
    pub fn xyzw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to w and `w` set to x
    #[inline]
    pub fn xywx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to w and `w` set to y
    #[inline]
    pub fn xywy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to w and `w` set to z
    #[inline]
    pub fn xywz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to y, `z` set to w and `w` set to w
    #[inline]
    pub fn xyww(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to x and `w` set to x
    #[inline]
    pub fn xzxx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to x and `w` set to y
    #[inline]
    pub fn xzxy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to x and `w` set to z
    #[inline]
    pub fn xzxz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to x and `w` set to w
    #[inline]
    pub fn xzxw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to y and `w` set to x
    #[inline]
    pub fn xzyx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to y and `w` set to y
    #[inline]
    pub fn xzyy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to y and `w` set to z
    #[inline]
    pub fn xzyz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to y and `w` set to w
    #[inline]
    pub fn xzyw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to z and `w` set to x
    #[inline]
    pub fn xzzx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to z and `w` set to y
    #[inline]
    pub fn xzzy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to z and `w` set to z
    #[inline]
    pub fn xzzz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to z and `w` set to w
    #[inline]
    pub fn xzzw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to w and `w` set to x
    #[inline]
    pub fn xzwx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to w and `w` set to y
    #[inline]
    pub fn xzwy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to w and `w` set to z
    #[inline]
    pub fn xzwz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to z, `z` set to w and `w` set to w
    #[inline]
    pub fn xzww(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.z,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to x and `w` set to x
    #[inline]
    pub fn xwxx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to x and `w` set to y
    #[inline]
    pub fn xwxy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to x and `w` set to z
    #[inline]
    pub fn xwxz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to x and `w` set to w
    #[inline]
    pub fn xwxw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to y and `w` set to x
    #[inline]
    pub fn xwyx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to y and `w` set to y
    #[inline]
    pub fn xwyy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to y and `w` set to z
    #[inline]
    pub fn xwyz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to y and `w` set to w
    #[inline]
    pub fn xwyw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to z and `w` set to x
    #[inline]
    pub fn xwzx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to z and `w` set to y
    #[inline]
    pub fn xwzy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to z and `w` set to z
    #[inline]
    pub fn xwzz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to z and `w` set to w
    #[inline]
    pub fn xwzw(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to w and `w` set to x
    #[inline]
    pub fn xwwx(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to w and `w` set to y
    #[inline]
    pub fn xwwy(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to w and `w` set to z
    #[inline]
    pub fn xwwz(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to x,
    /// `y` set to w, `z` set to w and `w` set to w
    #[inline]
    pub fn xwww(&self) -> Self {
        Vec4 {
            x: self.x,
            y: self.w,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to x and `w` set to x
    #[inline]
    pub fn yxxx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to x and `w` set to y
    #[inline]
    pub fn yxxy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to x and `w` set to z
    #[inline]
    pub fn yxxz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to x and `w` set to w
    #[inline]
    pub fn yxxw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to y and `w` set to x
    #[inline]
    pub fn yxyx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to y and `w` set to y
    #[inline]
    pub fn yxyy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to y and `w` set to z
    #[inline]
    pub fn yxyz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to y and `w` set to w
    #[inline]
    pub fn yxyw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to z and `w` set to x
    #[inline]
    pub fn yxzx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to z and `w` set to y
    #[inline]
    pub fn yxzy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to z and `w` set to z
    #[inline]
    pub fn yxzz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to z and `w` set to w
    #[inline]
    pub fn yxzw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to w and `w` set to x
    #[inline]
    pub fn yxwx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to w and `w` set to y
    #[inline]
    pub fn yxwy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to w and `w` set to z
    #[inline]
    pub fn yxwz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to x, `z` set to w and `w` set to w
    #[inline]
    pub fn yxww(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.x,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to x and `w` set to x
    #[inline]
    pub fn yyxx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to x and `w` set to y
    #[inline]
    pub fn yyxy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to x and `w` set to z
    #[inline]
    pub fn yyxz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to x and `w` set to w
    #[inline]
    pub fn yyxw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to y and `w` set to x
    #[inline]
    pub fn yyyx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to y and `w` set to y
    #[inline]
    pub fn yyyy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to y and `w` set to z
    #[inline]
    pub fn yyyz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to y and `w` set to w
    #[inline]
    pub fn yyyw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to z and `w` set to x
    #[inline]
    pub fn yyzx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to z and `w` set to y
    #[inline]
    pub fn yyzy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to z and `w` set to z
    #[inline]
    pub fn yyzz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to z and `w` set to w
    #[inline]
    pub fn yyzw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to w and `w` set to x
    #[inline]
    pub fn yywx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to w and `w` set to y
    #[inline]
    pub fn yywy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to w and `w` set to z
    #[inline]
    pub fn yywz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to y, `z` set to w and `w` set to w
    #[inline]
    pub fn yyww(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.y,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to x and `w` set to x
    #[inline]
    pub fn yzxx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to x and `w` set to y
    #[inline]
    pub fn yzxy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to x and `w` set to z
    #[inline]
    pub fn yzxz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to x and `w` set to w
    #[inline]
    pub fn yzxw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to y and `w` set to x
    #[inline]
    pub fn yzyx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to y and `w` set to y
    #[inline]
    pub fn yzyy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to y and `w` set to z
    #[inline]
    pub fn yzyz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to y and `w` set to w
    #[inline]
    pub fn yzyw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to z and `w` set to x
    #[inline]
    pub fn yzzx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to z and `w` set to y
    #[inline]
    pub fn yzzy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to z and `w` set to z
    #[inline]
    pub fn yzzz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to z and `w` set to w
    #[inline]
    pub fn yzzw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to w and `w` set to x
    #[inline]
    pub fn yzwx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to w and `w` set to y
    #[inline]
    pub fn yzwy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to w and `w` set to z
    #[inline]
    pub fn yzwz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to z, `z` set to w and `w` set to w
    #[inline]
    pub fn yzww(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.z,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to x and `w` set to x
    #[inline]
    pub fn ywxx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to x and `w` set to y
    #[inline]
    pub fn ywxy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to x and `w` set to z
    #[inline]
    pub fn ywxz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to x and `w` set to w
    #[inline]
    pub fn ywxw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to y and `w` set to x
    #[inline]
    pub fn ywyx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to y and `w` set to y
    #[inline]
    pub fn ywyy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to y and `w` set to z
    #[inline]
    pub fn ywyz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to y and `w` set to w
    #[inline]
    pub fn ywyw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to z and `w` set to x
    #[inline]
    pub fn ywzx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to z and `w` set to y
    #[inline]
    pub fn ywzy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to z and `w` set to z
    #[inline]
    pub fn ywzz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to z and `w` set to w
    #[inline]
    pub fn ywzw(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to w and `w` set to x
    #[inline]
    pub fn ywwx(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to w and `w` set to y
    #[inline]
    pub fn ywwy(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to w and `w` set to z
    #[inline]
    pub fn ywwz(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to y,
    /// `y` set to w, `z` set to w and `w` set to w
    #[inline]
    pub fn ywww(&self) -> Self {
        Vec4 {
            x: self.y,
            y: self.w,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to x and `w` set to x
    #[inline]
    pub fn zxxx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to x and `w` set to y
    #[inline]
    pub fn zxxy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to x and `w` set to z
    #[inline]
    pub fn zxxz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to x and `w` set to w
    #[inline]
    pub fn zxxw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to y and `w` set to x
    #[inline]
    pub fn zxyx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to y and `w` set to y
    #[inline]
    pub fn zxyy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to y and `w` set to z
    #[inline]
    pub fn zxyz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to y and `w` set to w
    #[inline]
    pub fn zxyw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to z and `w` set to x
    #[inline]
    pub fn zxzx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to z and `w` set to y
    #[inline]
    pub fn zxzy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to z and `w` set to z
    #[inline]
    pub fn zxzz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to z and `w` set to w
    #[inline]
    pub fn zxzw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to w and `w` set to x
    #[inline]
    pub fn zxwx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to w and `w` set to y
    #[inline]
    pub fn zxwy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to w and `w` set to z
    #[inline]
    pub fn zxwz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to x, `z` set to w and `w` set to w
    #[inline]
    pub fn zxww(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.x,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to x and `w` set to x
    #[inline]
    pub fn zyxx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to x and `w` set to y
    #[inline]
    pub fn zyxy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to x and `w` set to z
    #[inline]
    pub fn zyxz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to x and `w` set to w
    #[inline]
    pub fn zyxw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to y and `w` set to x
    #[inline]
    pub fn zyyx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to y and `w` set to y
    #[inline]
    pub fn zyyy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to y and `w` set to z
    #[inline]
    pub fn zyyz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to y and `w` set to w
    #[inline]
    pub fn zyyw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to z and `w` set to x
    #[inline]
    pub fn zyzx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to z and `w` set to y
    #[inline]
    pub fn zyzy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to z and `w` set to z
    #[inline]
    pub fn zyzz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to z and `w` set to w
    #[inline]
    pub fn zyzw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to w and `w` set to x
    #[inline]
    pub fn zywx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to w and `w` set to y
    #[inline]
    pub fn zywy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to w and `w` set to z
    #[inline]
    pub fn zywz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to y, `z` set to w and `w` set to w
    #[inline]
    pub fn zyww(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.y,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to x and `w` set to x
    #[inline]
    pub fn zzxx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to x and `w` set to y
    #[inline]
    pub fn zzxy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to x and `w` set to z
    #[inline]
    pub fn zzxz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to x and `w` set to w
    #[inline]
    pub fn zzxw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to y and `w` set to x
    #[inline]
    pub fn zzyx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to y and `w` set to y
    #[inline]
    pub fn zzyy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to y and `w` set to z
    #[inline]
    pub fn zzyz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to y and `w` set to w
    #[inline]
    pub fn zzyw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to z and `w` set to x
    #[inline]
    pub fn zzzx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to z and `w` set to y
    #[inline]
    pub fn zzzy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to z and `w` set to z
    #[inline]
    pub fn zzzz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to z and `w` set to w
    #[inline]
    pub fn zzzw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to w and `w` set to x
    #[inline]
    pub fn zzwx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to w and `w` set to y
    #[inline]
    pub fn zzwy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to w and `w` set to z
    #[inline]
    pub fn zzwz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to z, `z` set to w and `w` set to w
    #[inline]
    pub fn zzww(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.z,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to x and `w` set to x
    #[inline]
    pub fn zwxx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to x and `w` set to y
    #[inline]
    pub fn zwxy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to x and `w` set to z
    #[inline]
    pub fn zwxz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to x and `w` set to w
    #[inline]
    pub fn zwxw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to y and `w` set to x
    #[inline]
    pub fn zwyx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to y and `w` set to y
    #[inline]
    pub fn zwyy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to y and `w` set to z
    #[inline]
    pub fn zwyz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to y and `w` set to w
    #[inline]
    pub fn zwyw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to z and `w` set to x
    #[inline]
    pub fn zwzx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to z and `w` set to y
    #[inline]
    pub fn zwzy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to z and `w` set to z
    #[inline]
    pub fn zwzz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to z and `w` set to w
    #[inline]
    pub fn zwzw(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to w and `w` set to x
    #[inline]
    pub fn zwwx(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to w and `w` set to y
    #[inline]
    pub fn zwwy(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to w and `w` set to z
    #[inline]
    pub fn zwwz(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to z,
    /// `y` set to w, `z` set to w and `w` set to w
    #[inline]
    pub fn zwww(&self) -> Self {
        Vec4 {
            x: self.z,
            y: self.w,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to x and `w` set to x
    #[inline]
    pub fn wxxx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to x and `w` set to y
    #[inline]
    pub fn wxxy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to x and `w` set to z
    #[inline]
    pub fn wxxz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to x and `w` set to w
    #[inline]
    pub fn wxxw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to y and `w` set to x
    #[inline]
    pub fn wxyx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to y and `w` set to y
    #[inline]
    pub fn wxyy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to y and `w` set to z
    #[inline]
    pub fn wxyz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to y and `w` set to w
    #[inline]
    pub fn wxyw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to z and `w` set to x
    #[inline]
    pub fn wxzx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to z and `w` set to y
    #[inline]
    pub fn wxzy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to z and `w` set to z
    #[inline]
    pub fn wxzz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to z and `w` set to w
    #[inline]
    pub fn wxzw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to w and `w` set to x
    #[inline]
    pub fn wxwx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to w and `w` set to y
    #[inline]
    pub fn wxwy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to w and `w` set to z
    #[inline]
    pub fn wxwz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to x, `z` set to w and `w` set to w
    #[inline]
    pub fn wxww(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.x,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to x and `w` set to x
    #[inline]
    pub fn wyxx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to x and `w` set to y
    #[inline]
    pub fn wyxy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to x and `w` set to z
    #[inline]
    pub fn wyxz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to x and `w` set to w
    #[inline]
    pub fn wyxw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to y and `w` set to x
    #[inline]
    pub fn wyyx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to y and `w` set to y
    #[inline]
    pub fn wyyy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to y and `w` set to z
    #[inline]
    pub fn wyyz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to y and `w` set to w
    #[inline]
    pub fn wyyw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to z and `w` set to x
    #[inline]
    pub fn wyzx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to z and `w` set to y
    #[inline]
    pub fn wyzy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to z and `w` set to z
    #[inline]
    pub fn wyzz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to z and `w` set to w
    #[inline]
    pub fn wyzw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to w and `w` set to x
    #[inline]
    pub fn wywx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to w and `w` set to y
    #[inline]
    pub fn wywy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to w and `w` set to z
    #[inline]
    pub fn wywz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to y, `z` set to w and `w` set to w
    #[inline]
    pub fn wyww(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.y,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to x and `w` set to x
    #[inline]
    pub fn wzxx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to x and `w` set to y
    #[inline]
    pub fn wzxy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to x and `w` set to z
    #[inline]
    pub fn wzxz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to x and `w` set to w
    #[inline]
    pub fn wzxw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to y and `w` set to x
    #[inline]
    pub fn wzyx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to y and `w` set to y
    #[inline]
    pub fn wzyy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to y and `w` set to z
    #[inline]
    pub fn wzyz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to y and `w` set to w
    #[inline]
    pub fn wzyw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to z and `w` set to x
    #[inline]
    pub fn wzzx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to z and `w` set to y
    #[inline]
    pub fn wzzy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to z and `w` set to z
    #[inline]
    pub fn wzzz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to z and `w` set to w
    #[inline]
    pub fn wzzw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to w and `w` set to x
    #[inline]
    pub fn wzwx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to w and `w` set to y
    #[inline]
    pub fn wzwy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to w and `w` set to z
    #[inline]
    pub fn wzwz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to z, `z` set to w and `w` set to w
    #[inline]
    pub fn wzww(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.z,
            z: self.w,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to x and `w` set to x
    #[inline]
    pub fn wwxx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.x,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to x and `w` set to y
    #[inline]
    pub fn wwxy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.x,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to x and `w` set to z
    #[inline]
    pub fn wwxz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.x,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to x and `w` set to w
    #[inline]
    pub fn wwxw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.x,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to y and `w` set to x
    #[inline]
    pub fn wwyx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.y,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to y and `w` set to y
    #[inline]
    pub fn wwyy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.y,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to y and `w` set to z
    #[inline]
    pub fn wwyz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.y,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to y and `w` set to w
    #[inline]
    pub fn wwyw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.y,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to z and `w` set to x
    #[inline]
    pub fn wwzx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.z,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to z and `w` set to y
    #[inline]
    pub fn wwzy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.z,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to z and `w` set to z
    #[inline]
    pub fn wwzz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.z,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to z and `w` set to w
    #[inline]
    pub fn wwzw(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.z,
            w: self.w
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to w and `w` set to x
    #[inline]
    pub fn wwwx(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.x
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to w and `w` set to y
    #[inline]
    pub fn wwwy(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.y
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to w and `w` set to z
    #[inline]
    pub fn wwwz(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.z
        }
    }
    /// "Swizzle-like" method returning a new Vec4<f32> with `x` set to w,
    /// `y` set to w, `z` set to w and `w` set to w
    #[inline]
    pub fn wwww(&self) -> Self {
        Vec4 {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.w
        }
    }

    /// Returns this Vec4<f32> as a 3 element array
    #[inline]
    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

/// Calc methods for Vec4<f32>
impl Vec4<f32> {
    /// Calculates the dot product of two Vec4<f32>
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z + self.w*rhs.w
    }
    /// Calculates the square-length of this Vec4<f32>
    ///
    /// This can be used for faster comparisons of lengths of vectors, but is
    /// not a replacement for [len](#method.len).
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.dot(self)
    }
    /// Calculates the length of this Vec4<f32>
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }
    /// Calculates the square-distance between two Vec4<f32>
    ///
    /// This can be used for faster comparisons of distances, but is not a
    /// replacement for [dist](#method.dist)
    #[inline]
    pub fn sqr_dist(&self, b: &Self) -> f32 {
        (*self - *b).sqr_len()
    }
    /// Calculates the distance between two Vec4<f32>
    #[inline]
    pub fn dist(&self, b: &Self) -> f32 {
        (*self - *b).len()
    }
    /// Calculates and returns a normalized copy of this Vec4<f32>
    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.len();

        Vec4 {
            x: self.x/len,
            y: self.y/len,
            z: self.z/len,
            w: self.w/len
        }
    }
}

/// Mutate methods for Vec4<f32>
impl Vec4<f32> {
    /// Sets a mutable Vec4<f32> to values x, y, z and w
    #[inline]
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
    /// Sets a mutable Vec4<f32> to values in a three element f32 array
    #[inline]
    pub fn set_array(&mut self, a: [f32; 4]) {
        self.x = a[0];
        self.y = a[1];
        self.z = a[2];
        self.w = a[3];
    }
}

impl Mul for Vec4<f32> {
    type Output = Vec4<f32>;
    fn mul(self, rhs: Vec4<f32>) -> Vec4<f32> {
        Vec4 {
            x: self.x*rhs.x,
            y: self.y*rhs.y,
            z: self.z*rhs.z,
            w: self.w*rhs.w
        }
    }
}
impl Div for Vec4<f32> {
    type Output = Vec4<f32>;
    fn div(self, rhs: Vec4<f32>) -> Vec4<f32> {
        Vec4 {
            x: self.x/rhs.x,
            y: self.y/rhs.y,
            z: self.z/rhs.z,
            w: self.w/rhs.w
        }
    }
}
impl Add for Vec4<f32> {
    type Output = Vec4<f32>;
    fn add(self, rhs: Vec4<f32>) -> Vec4<f32> {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}
impl Sub for Vec4<f32> {
    type Output = Vec4<f32>;
    fn sub(self, rhs: Vec4<f32>) -> Vec4<f32> {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

#[cfg(test)]
mod tests {
}
