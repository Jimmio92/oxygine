use std::ops::{Index, IndexMut, Mul, MulAssign, Div, DivAssign, Add, AddAssign,
               Sub, SubAssign};

#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
/// Vec3 stores 3 elements of any type with specialization for f32
///
/// Vec3 is used to store a 3 dimensional point or direction when using f32
/// as the type. This then provides more useful things (such as
/// [`dot`](#method.dot) product, and directional constructors such as
/// [`left`](#method.left) and [`right`](#method.right).
pub struct Vec3<T> {
    /// The x coordinate/element (index 0)
    pub x: T,
    /// The y coordinate/element (index 1)
    pub y: T,
    /// The z coordinate/element (index 2)
    pub z: T
}

/// Generic Vec3 methods
impl<T> Vec3<T> where T: Copy {
    /// Constructs a new Vec3 from values `x`, `y` and `z`
    pub fn from(x: T, y: T, z: T) -> Self {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }
    /// Constructs a new Vec3 from a 3 element array in [x, y, z] order
    pub fn from_array(a: [T; 3]) -> Self {
        Vec3 {
            x: a[0],
            y: a[1],
            z: a[2]
        }
    }
    /// Returns a Vec3 as a 3 element array in [x, y, z] order
    pub fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    /// Sets a Vec3 to the values x = `x`, y = `y`, and z = `z`
    pub fn set(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    /// Sets a Vec3 to the values of a 3 element array in [x, y, z] order
    pub fn set_array(&mut self, a: [T; 3]) {
        self.x = a[0];
        self.y = a[1];
        self.z = a[2];
    }
}

include!(concat!(env!("OUT_DIR"), "/vec3_swizzle.rs"));

/// f32 specialized Vec3 methods
impl Vec3<f32> {
    /// Returns a new Vec3 with x set to 0, y set to 0 and z set to 0
    pub fn zero() -> Self {
        Vec3::from(0.0f32, 0.0f32, 0.0f32)
    }
    /// Returns a new Vec3 with x set to 1, y set to 1 and z set to 1
    pub fn one() -> Self {
        Vec3::from(1.0f32, 1.0f32, 1.0f32)
    }
    /// Returns a new Vec3 with x set to -1, y set to 0 and z set to 0
    pub fn left() -> Self {
        Vec3::from(-1.0f32, 0.0f32, 0.0f32)
    }
    /// Returns a new Vec3 with x set to 1, y set to 0 and z set to 0
    pub fn right() -> Self {
        Vec3::from(1.0f32, 0.0f32, 0.0f32)
    }
    /// Returns a new Vec3 with x set to 0, y set to -1, and z set to 0
    pub fn down() -> Self {
        Vec3::from(0.0f32, -1.0f32, 0.0f32)
    }
    /// Returns a new Vec3 with x set to 0, y set to 1 and z set to 0
    pub fn up() -> Self {
        Vec3::from(0.0f32, 1.0f32, 0.0f32)
    }
    /// Returns a new Vec3 with x set to 0, y set to 0 and z set to -1
    pub fn backward() -> Self {
        Vec3::from(0.0f32, 0.0f32, -1.0f32)
    }
    /// Returns a new Vec3 with x set to 0, y set to 0 and z set to 1
    pub fn forward() -> Self {
        Vec3::from(0.0f32, 0.0f32, 1.0f32)
    }

    /// Calculates the dot product of a Vec3
    pub fn dot(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    /// Calculates the cross product of two Vec3s
    pub fn cross(&self, b: Self) -> Self {
        Vec3::from(
            self.y*b.z - self.z*b.y,
            self.z*b.x - self.x*b.z,
            self.x*b.y - self.y*b.x
        )
    }
    /// Calculates the length squared of a Vec3
    ///
    /// Note that the actual length of a Vec3 requires a square root operation
    /// that this method purposely avoids for faster comparison of lengths.
    pub fn length_squared(&self) -> f32 {
        self.dot()
    }
    /// Calculates the length of a Vec3
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    /// Returns a new Vec3 with values normalized (unit length)
    pub fn normalized(&self) -> Self {
        let len = self.length();
        Vec3::from(self.x/len, self.y/len, self.z/len)
    }
    /// Mutates self to become normalized (unit length)
    pub fn normalize_self(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
    /// Calculates the distance squared between two Vec3s
    ///
    /// Note that the actual distance between two Vec3s requires a square root
    /// operation that this method purposely avoids for faster comparison of
    /// distances.
    pub fn distance_squared(&self, b: &Self) -> f32 {
        (*self - *b).length_squared()
    }
    /// Calculates the distance between two Vec3s
    pub fn distance(&self, b: &Self) -> f32 {
        (*self - *b).length()
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Attempted to index Vec3<T> out of range!")
        }
    }
}
impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Attempted to index_mut Vec3<T> out of range!")
        }
    }
}
impl<T> Mul<Self> for Vec3<T> where T: Mul<T, Output = T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::from(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z)
    }
}
impl<T> Mul<T> for Vec3<T> where T: Mul<T, Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec3::from(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}
impl<T> MulAssign<Self> for Vec3<T> where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl<T> MulAssign<T> for Vec3<T> where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl<T> Div<Self> for Vec3<T> where T: Div<T, Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::from(self.x/rhs.x, self.y/rhs.y, self.z/rhs.z)
    }
}
impl<T> Div<T> for Vec3<T> where T: Div<T, Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3::from(self.x/rhs, self.y/rhs, self.z/rhs)
    }
}
impl<T> DivAssign<Self> for Vec3<T> where T: DivAssign<T> + Copy {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl<T> DivAssign<T> for Vec3<T> where T: DivAssign<T> + Copy {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl<T> Add<Self> for Vec3<T> where T: Add<T, Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::from(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl<T> Add<T> for Vec3<T> where T: Add<T, Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Vec3::from(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}
impl<T> AddAssign<Self> for Vec3<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<T> AddAssign<T> for Vec3<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
impl<T> Sub<Self> for Vec3<T> where T: Sub<T, Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::from(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl<T> Sub<T> for Vec3<T> where T: Sub<T, Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3::from(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}
impl<T> SubAssign<Self> for Vec3<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<T> SubAssign<T> for Vec3<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
