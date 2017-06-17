use std::ops::{Index, IndexMut, Mul, MulAssign, Div, DivAssign, Add, AddAssign,
               Sub, SubAssign};

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub struct Vec2<T> {
    x: T,
    y: T
}

/// Vec2<T> methods
impl<T> Vec2<T> where T: Copy {
    fn from(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y
        }
    }
    fn from_array(a: [T; 2]) -> Self {
        Vec2 {
            x: a[0],
            y: a[1]
        }
    }
    fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
    fn set_array(&mut self, a: [T; 2]) {
        self.x = a[0];
        self.y = a[1];
    }

    fn x(&self) -> T {
        self.x
    }
    fn y(&self) -> T {
        self.y
    }
    fn xx(&self) -> Self {
        Vec2::from(self.x, self.x)
    }
    fn xy(&self) -> Self {
        Vec2::from(self.x, self.y)
    }
    fn yx(&self) -> Self {
        Vec2::from(self.y, self.x)
    }
    fn yy(&self) -> Self {
        Vec2::from(self.y, self.y)
    }
}

/// Vec2<f32> methods
impl Vec2<f32> {
    fn zero() -> Self {
        Vec2::from(0.0f32, 0.0f32)
    }
    fn one() -> Self {
        Vec2::from(1.0f32, 1.0f32)
    }
    fn left() -> Self {
        Vec2::from(-1.0f32, 0.0f32)
    }
    fn right() -> Self {
        Vec2::from(1.0f32, 0.0f32)
    }
    fn down() -> Self {
        Vec2::from(0.0f32, -1.0f32)
    }
    fn up() -> Self {
        Vec2::from(0.0f32, 1.0f32)
    }

    fn dot(&self) -> f32 {
        self.x*self.x + self.y*self.y
    }
    fn length_squared(&self) -> f32 {
        self.dot()
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn normalized(&self) -> Self {
        let len = self.length();
        Vec2::from(self.x/len, self.y/len)
    }
    fn normalize_self(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }
    fn distance(&self, b: &Self) -> f32 {
        (*self - *b).length()
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            _ => &self.y
        }
    }
}
impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            _ => &mut self.y
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
