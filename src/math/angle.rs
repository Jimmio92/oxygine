extern crate num;
use self::num::Float;
use std::ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign};

#[derive(Debug,Copy,Clone,Mul,MulAssign,Div,DivAssign,Add,AddAssign,Sub,SubAssign)]
/// Stores an angle in either degrees or radians
pub enum Angle<T> where T: Float + Into<T> {
    /// Radians
    Radians(T),
    /// Degrees
    Degrees(T)
}

/// Creates a new Angle in radians
pub fn rad<T>(angle: T) -> Angle<T> where T: Float { Angle::Radians(angle) }
/// Creates a new Angle in degrees
pub fn deg<T>(angle: T) -> Angle<T> where T: Float { Angle::Degrees(angle) }

impl<T> Angle<T> where T: Float + Into<T> {
    /// Converts self to radians
    pub fn becomes_radians(&mut self) {
        *self = match *self {
            Angle::Radians(a) => Angle::Radians(a),
            Angle::Degrees(a) => Angle::Radians(a*T::from(0.01745329251994329577).unwrap())
        };
    }
    /// Converts self to degrees
    pub fn becomes_degrees(&mut self) {
        *self = match *self {
            Angle::Radians(a) => Angle::Degrees(a*T::from(57.2957795130823208768).unwrap()),
            Angle::Degrees(a) => Angle::Degrees(a)
        };
    }

    /// Returns this Angle in radians
    pub fn to_radians(&self) -> Self {
        match *self {
            Angle::Radians(a) => Angle::Radians(a),
            Angle::Degrees(a) => Angle::Radians(a*T::from(0.01745329251994329577).unwrap())
        }
    }
    /// Returns this Angle in degrees
    pub fn to_degrees(&self) -> Self {
        match *self {
            Angle::Radians(a) => Angle::Degrees(a*T::from(57.2957795130823208768).unwrap()),
            Angle::Degrees(a) => Angle::Degrees(a)
        }
    }
}
