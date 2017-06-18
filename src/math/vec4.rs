use std::ops::{Index, IndexMut, Mul, MulAssign, Div, DivAssign, Add, AddAssign,
               Sub, SubAssign};

#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
/// Vec4 stores 4 elements of any type with specialization for f32
///
/// Vec4 is used to store a 4 dimensional point or direction when using f32
/// as the type. This then provides more useful things (such as
/// [`dot`](#method.dot) product, and directional constructors such as
/// [`left`](#method.left) and [`right`](#method.right).
pub struct Vec4<T> {
    /// The x coordinate/element (index 0)
    pub x: T,
    /// The y coordinate/element (index 1)
    pub y: T,
    /// The z coordinate/element (index 2)
    pub z: T,
    /// The w coordinate/element (index 3)
    pub w: T
}

/// Generic Vec4 methods
impl<T> Vec4<T> where T: Copy {
    /// Constructs a new Vec4 from values `x`, `y`, `z` and `w`
    pub fn from(x: T, y: T, z: T, w: T) -> Self {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
    /// Constructs a new Vec4 from a 4 element array in [x, y, z, w] order
    pub fn from_array(a: [T; 4]) -> Self {
        Vec4 {
            x: a[0],
            y: a[1],
            z: a[2],
            w: a[3]
        }
    }
    /// Returns a Vec4 as a 4 element array in [x, y, z, w] order
    pub fn to_array(&self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Sets a Vec4 to the values x = `x`, y = `y`, z = `z` and w = `w`
    pub fn set(&mut self, x: T, y: T, z: T, w: T) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
    /// Sets a Vec4 to the values of a 4 element array in [x, y, z, w] order
    pub fn set_array(&mut self, a: [T; 4]) {
        self.x = a[0];
        self.y = a[1];
        self.z = a[2];
        self.w = a[3];
    }
}

include!(concat!(env!("OUT_DIR"), "/vec4_swizzle.rs"));

/// f32 specialized Vec4 methods
impl Vec4<f32> {
    /// Returns a new Vec4 with x set to 0, y set to 0, z set to 0 and w set to
    /// 1
    pub fn zero() -> Self {
        Vec4::from(0.0f32, 0.0f32, 0.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to 1, y set to 1, z set to 1 and w set to
    /// 1
    pub fn one() -> Self {
        Vec4::from(1.0f32, 1.0f32, 1.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to -1, y set to 0, z set to 0 and w set to
    /// 1
    pub fn left() -> Self {
        Vec4::from(-1.0f32, 0.0f32, 0.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to 1, y set to 0, z set to 0 and w set to
    /// 1
    pub fn right() -> Self {
        Vec4::from(1.0f32, 0.0f32, 0.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to 0, y set to -1, z set to 0 and w set to
    /// 1
    pub fn down() -> Self {
        Vec4::from(0.0f32, -1.0f32, 0.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to 0, y set to 1, z set to 0 and w set to
    /// 1
    pub fn up() -> Self {
        Vec4::from(0.0f32, 1.0f32, 0.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to 0, y set to 0, z set to -1 and w set to
    /// 1
    pub fn backward() -> Self {
        Vec4::from(0.0f32, 0.0f32, -1.0f32, 1.0f32)
    }
    /// Returns a new Vec4 with x set to 0, y set to 0, z set to 1 and w set to
    /// 1
    pub fn forward() -> Self {
        Vec4::from(0.0f32, 0.0f32, 1.0f32, 1.0f32)
    }

    /// Calculates the dot product of a Vec4
    pub fn dot(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    /// Calculates the cross product of three Vec4s
    fn cross(&self, b: Self, c: Self) -> Self {
        Vec4::from(
             (self.y*(b.z*c.w - c.z*b.w) - self.z*(b.y*c.w - c.y*b.w)
                + self.w*(b.y*c.z - c.y*b.z)),
            -(self.x*(b.z*c.w - c.z*b.w) - self.z*(b.x*c.w - c.x*b.w)
                + self.w*(b.x*c.z - c.x*b.z)),
             (self.x*(b.y*c.w - c.y*b.w) - self.y*(b.x*c.w - c.x*b.w)
                + self.w*(b.x*c.y - c.x*b.y)),
            -(self.x*(b.y*c.z - c.y*b.z) - self.y*(b.x*c.z - c.x*b.z)
                + self.w*(b.x*c.y - c.y*b.y))
        )
    }
    /// Calculates the length squared of a Vec4
    ///
    /// Note that the actual length of a Vec4 requires a square root operation
    /// that this method purposely avoids for faster comparison of lengths.
    pub fn length_squared(&self) -> f32 {
        self.dot()
    }
    /// Calculates the length of a Vec4
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    /// Returns a new Vec4 with values normalized (unit length)
    pub fn normalized(&self) -> Self {
        let len = self.length();
        Vec4::from(self.x/len, self.y/len, self.z/len, self.w/len)
    }
    /// Mutates self to become normalized (unit length)
    pub fn normalize_self(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self.w /= len;
    }
    /// Calculates the distance squared between two Vec4s
    ///
    /// Note that the actual distance between two Vec4s requires a square root
    /// operation that this method purposely avoids for faster comparison of
    /// distances.
    pub fn distance_squared(&self, b: &Self) -> f32 {
        (*self - *b).length_squared()
    }
    /// Calculates the distance between two Vec4s
    pub fn distance(&self, b: &Self) -> f32 {
        (*self - *b).length()
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Attempted to index Vec4<T> out of range!")
        }
    }
}
impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Attempted to index_mut Vec4<T> out of range!")
        }
    }
}
impl<T> Mul<Self> for Vec4<T> where T: Mul<T, Output = T> + Copy {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Vec4<T> {
        Vec4::from(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z, self.w*rhs.w)
    }
}
impl<T> Mul<T> for Vec4<T> where T: Mul<T, Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec4::from(self.x*rhs, self.y*rhs, self.z*rhs, self.w*rhs)
    }
}
impl<T> MulAssign<Self> for Vec4<T> where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: Vec4<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}
impl<T> MulAssign<T> for Vec4<T> where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
impl<T> Div<Self> for Vec4<T> where T: Div<T, Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec4::from(self.x/rhs.x, self.y/rhs.y, self.z/rhs.z, self.w/rhs.w)
    }
}
impl<T> Div<T> for Vec4<T> where T: Div<T, Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec4::from(self.x/rhs, self.y/rhs, self.z/rhs, self.w/rhs)
    }
}
impl<T> DivAssign<Self> for Vec4<T> where T: DivAssign<T> + Copy {
    fn div_assign(&mut self, rhs: Vec4<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}
impl<T> DivAssign<T> for Vec4<T> where T: DivAssign<T> + Copy {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
impl<T> Add<Self> for Vec4<T> where T: Add<T, Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4::from(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w
        )
    }
}
impl<T> Add<T> for Vec4<T> where T: Add<T, Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Vec4::from(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs)
    }
}
impl<T> AddAssign<Self> for Vec4<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: Vec4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
impl<T> AddAssign<T> for Vec4<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}
impl<T> Sub<Self> for Vec4<T> where T: Sub<T, Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4::from(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w
        )
    }
}
impl<T> Sub<T> for Vec4<T> where T: Sub<T, Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Vec4::from(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs)
    }
}
impl<T> SubAssign<Self> for Vec4<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: Vec4<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
impl<T> SubAssign<T> for Vec4<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}
