use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use cgmath::{InnerSpace, Zero};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec4(pub cgmath::Vector4<f32>);

impl Default for Vec4 {
    fn default() -> Self {
        Self(cgmath::Vector4::zero())
    }
}

impl Deref for Vec4 {
    type Target = cgmath::Vector4<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(cgmath::Vector4::new(x, y, z, w))
    }

    pub fn zero() -> Self {
        Self(cgmath::Vector4::zero())
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }

    pub fn magnitude2(&self) -> f32 {
        self.0.magnitude2()
    }

    pub fn normalize(&self) -> Self {
        if self.magnitude() == 0.0 {
            Self::zero()
        } else {
            Self(self.0.normalize())
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.0.dot(other.0)
    }

    pub const RED: Self = Self(cgmath::Vector4::new(1.0, 0.0, 0.0, 1.0));
    pub const GREEN: Self = Self(cgmath::Vector4::new(0.0, 1.0, 0.0, 1.0));
    pub const BLUE: Self = Self(cgmath::Vector4::new(0.0, 0.0, 1.0, 1.0));
    pub const WHITE: Self = Self(cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0));
    pub const BLACK: Self = Self(cgmath::Vector4::new(0.0, 0.0, 0.0, 1.0));
}

// Conversions
impl From<cgmath::Vector4<f32>> for Vec4 {
    fn from(v: cgmath::Vector4<f32>) -> Self {
        Self(v)
    }
}

impl From<Vec4> for cgmath::Vector4<f32> {
    fn from(v: Vec4) -> Self {
        v.0
    }
}

// Operators
impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self(self.0 * scalar)
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    fn mul(self, vec: Vec4) -> Vec4 {
        Vec4(vec.0 * self)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self(self.0 / scalar)
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, scalar: f32) {
        self.0 /= scalar;
    }
}

impl Neg for Vec4 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}