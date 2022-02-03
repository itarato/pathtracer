use super::defs::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! impl_arithmetics {
    ($traitname:ident, $fname:ident, $op:tt) => {
        impl $traitname for Vec3 {
            type Output = Self;
            fn $fname(self, other: Self) -> Self::Output {
                Self::new(self.x $op other.x, self.y $op other.y, self.z $op other.z)
            }
        }
    };
}

macro_rules! impl_assign_arithmetics {
    ($traitname:ident, $fname:ident, $op:tt) => {
        impl $traitname for Vec3 {
            fn $fname(&mut self, other: Self) {
                *self = Self::new(self.x $op other.x, self.y $op other.y, self.z $op other.z)
            }
        }
    };
}

macro_rules! impl_flat_arithmetics {
    ($traitname:ident, $fname:ident, $op:tt) => {
        impl $traitname<FloatT> for Vec3 {
            type Output = Self;
            fn $fname(self, other: FloatT) -> Self::Output {
                Self::new(self.x $op other, self.y $op other, self.z $op other)
            }
        }
    };
}

pub trait Upscale<T> {
    fn upscale(&self) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: FloatT,
    pub y: FloatT,
    pub z: FloatT,
}

impl Vec3 {
    pub fn new(x: FloatT, y: FloatT, z: FloatT) -> Self {
        Self { x, y, z }
    }

    pub fn unif(v: FloatT) -> Self {
        Self::new(v, v, v)
    }

    pub fn dot(lhs: Self, rhs: Self) -> FloatT {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self::new(
            lhs.y * rhs.z - lhs.z * rhs.y,
            -(lhs.x * rhs.z - lhs.z * rhs.x),
            lhs.x * rhs.y - lhs.y * rhs.x,
        )
    }

    pub fn at(&self, idx: usize) -> FloatT {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(),
        }
    }

    pub fn squared_len(&self) -> FloatT {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> FloatT {
        self.squared_len().sqrt()
    }

    // Ref: `vec3::make_unit_vector`.
    // This "unit" thing is confusing.
    pub fn into_unit(&mut self) {
        let k: FloatT = 1.0 as FloatT / self.len();
        *self = *self * k;
    }

    // Ref: `unit_vector(vec3)`.
    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }
}

impl Upscale<u8> for FloatT {
    fn upscale(&self) -> u8 {
        (*self * 255.99) as u8
    }
}

impl_arithmetics!(Add, add, +);
impl_arithmetics!(Sub, sub, -);
impl_arithmetics!(Mul, mul, *);
impl_arithmetics!(Div, div, /);
impl_assign_arithmetics!(AddAssign, add_assign, +);
impl_assign_arithmetics!(SubAssign, sub_assign, -);
impl_assign_arithmetics!(MulAssign, mul_assign, *);
impl_assign_arithmetics!(DivAssign, div_assign, /);
impl_flat_arithmetics!(Add, add, +);
impl_flat_arithmetics!(Sub, sub, -);
impl_flat_arithmetics!(Mul, mul, *);
impl_flat_arithmetics!(Div, div, /);

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn can_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        let sum = v1 + v2;
        assert_eq!(3.0, sum.x);
        assert_eq!(5.0, sum.y);
        assert_eq!(7.0, sum.z);
    }
}
