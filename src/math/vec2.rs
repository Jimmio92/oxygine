use std::ops::{Index, IndexMut, Mul, MulAssign, Div, DivAssign, Add, AddAssign,
               Sub, SubAssign};

#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
/// Vec2 stores 2 elements of any type with specialization for f32
///
/// Vec2 is used to store a 2 dimensional point or direction when using f32
/// as the type. This then provides more useful things (such as
/// [`dot`](#method.dot) product, and directional constructors such as
/// [`left`](#method.left) and [`right`](#method.right).
pub struct Vec2<T> {
    /// The x coordinate/element (index 0)
    pub x: T,
    /// The y coordinate/element (index 1)
    pub y: T
}

/// Generic Vec2 methods
impl<T> Vec2<T> where T: Copy {
    /// Constructs a new Vec2 from values `x` and `y`
    pub fn from(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y
        }
    }
    /// Constructs a new Vec2 from a 2 element array in [x, y] order
    pub fn from_array(a: [T; 2]) -> Self {
        Vec2 {
            x: a[0],
            y: a[1]
        }
    }
    /// Returns a Vec2 as a 2 element array in [x, y] order
    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    /// Sets a Vec2 to the values x = `x` and y = `y`
    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
    /// Sets a Vec2 to the values of a 2 element array in [x, y] order
    pub fn set_array(&mut self, a: [T; 2]) {
        self.x = a[0];
        self.y = a[1];
    }
}

include!(concat!(env!("OUT_DIR"), "/vec2_swizzle.rs"));

/// f32 specialized Vec2 methods
impl Vec2<f32> {
    /// Returns a new Vec2 with x set to 0 and y set to 0
    pub fn zero() -> Self {
        Vec2::from(0.0f32, 0.0f32)
    }
    /// Returns a new Vec2 with x set to 1 and y set to 1
    pub fn one() -> Self {
        Vec2::from(1.0f32, 1.0f32)
    }
    /// Returns a new Vec2 with x set to -1 and y set to 0
    pub fn left() -> Self {
        Vec2::from(-1.0f32, 0.0f32)
    }
    /// Returns a new Vec2 with x set to 1 and y set to 0
    pub fn right() -> Self {
        Vec2::from(1.0f32, 0.0f32)
    }
    /// Returns a new Vec2 with x set to 0 and y set to -1
    pub fn down() -> Self {
        Vec2::from(0.0f32, -1.0f32)
    }
    /// Returns a new Vec2 with x set to 0 and y set to 1
    pub fn up() -> Self {
        Vec2::from(0.0f32, 1.0f32)
    }

    /// Calculates the dot product of a Vec2
    pub fn dot(&self) -> f32 {
        self.x*self.x + self.y*self.y
    }
    /// Calculates the length squared of a Vec2
    ///
    /// Note that the actual length of a Vec2 requires a square root operation
    /// that this method purposely avoids for faster comparison of lengths.
    pub fn length_squared(&self) -> f32 {
        self.dot()
    }
    /// Calculates the length of a Vec2
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    /// Returns a new Vec2 with values normalized (unit length)
    pub fn normalized(&self) -> Self {
        let len = self.length();
        Vec2::from(self.x/len, self.y/len)
    }
    /// Mutates self to become normalized (unit length)
    pub fn normalize_self(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }
    /// Calculates the distance squared between two Vec2s
    ///
    /// Note that the actual distance between two Vec2s requires a square root
    /// operation that this method purposely avoids for faster comparison of
    /// distances.
    pub fn distance_squared(&self, b: &Self) -> f32 {
        (*self - *b).length_squared()
    }
    /// Calculates the distance between two Vec2s
    pub fn distance(&self, b: &Self) -> f32 {
        (*self - *b).length()
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Attempted to index Vec2<T> out of range!")
        }
    }
}
impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Attempted to index_mut Vec2<T> out of range!")
        }
    }
}
impl<T> Mul<Self> for Vec2<T> where T: Mul<T, Output = T> + Copy {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::from(self.x*rhs.x, self.y*rhs.y)
    }
}
impl<T> Mul<T> for Vec2<T> where T: Mul<T, Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec2::from(self.x*rhs, self.y*rhs)
    }
}
impl<T> MulAssign<Self> for Vec2<T> where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl<T> MulAssign<T> for Vec2<T> where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl<T> Div<Self> for Vec2<T> where T: Div<T, Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2::from(self.x/rhs.x, self.y/rhs.y)
    }
}
impl<T> Div<T> for Vec2<T> where T: Div<T, Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec2::from(self.x/rhs, self.y/rhs)
    }
}
impl<T> DivAssign<Self> for Vec2<T> where T: DivAssign<T> + Copy {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl<T> DivAssign<T> for Vec2<T> where T: DivAssign<T> + Copy {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
impl<T> Add<Self> for Vec2<T> where T: Add<T, Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::from(self.x + rhs.x, self.y + rhs.y)
    }
}
impl<T> Add<T> for Vec2<T> where T: Add<T, Output = T> + Copy {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Vec2::from(self.x + rhs, self.y + rhs)
    }
}
impl<T> AddAssign<Self> for Vec2<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T> AddAssign<T> for Vec2<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}
impl<T> Sub<Self> for Vec2<T> where T: Sub<T, Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::from(self.x - rhs.x, self.y - rhs.y)
    }
}
impl<T> Sub<T> for Vec2<T> where T: Sub<T, Output = T> + Copy {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Vec2::from(self.x - rhs, self.y - rhs)
    }
}
impl<T> SubAssign<Self> for Vec2<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T> SubAssign<T> for Vec2<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}
