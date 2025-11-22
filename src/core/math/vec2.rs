use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use cgmath::{Basis2, Deg, InnerSpace, MetricSpace, Rad, Rotation, Rotation2, Zero};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2(pub cgmath::Vector2<f32>);

impl Default for Vec2 {
    fn default() -> Self {
        Self(cgmath::Vector2::zero())
    }
}

impl Deref for Vec2 {
    type Target = cgmath::Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(cgmath::Vector2::new(x, y))
    }

    pub fn zero() -> Self {
        Self(cgmath::Vector2::zero())
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
    
    pub fn distance(&self, other: Self) -> f32 {
        self.0.distance(other.0)
    }

    pub fn distance2(&self, other: Self) -> f32 {
        self.0.distance2(other.0)
    }

    pub fn min(a: Vec2, b: Vec2) -> Vec2 {
        Vec2::new(a.x.min(b.x), a.y.min(b.y))
    }

    pub fn max(a: Vec2, b: Vec2) -> Vec2 {
        Vec2::new(a.x.max(b.x), a.y.max(b.y))
    }

    pub fn rotate_deg(vec: Vec2, angle_deg: f32) -> Vec2 {
        let rotation: Basis2<f32> = Rotation2::from_angle(Deg(angle_deg));
        let rotated_vector = rotation.rotate_vector(vec.0);
        Vec2(rotated_vector)
    }

    pub fn rotate_rad(vec: Vec2, angle_rad: f32) -> Vec2 {
        let rotation: Basis2<f32> = Rotation2::from_angle(Rad(angle_rad));
        let rotated_vector = rotation.rotate_vector(vec.0);
        Vec2(rotated_vector)
    }

}

// Conversions
impl From<cgmath::Vector2<f32>> for Vec2 {
    fn from(v: cgmath::Vector2<f32>) -> Self {
        Self(v)
    }
}

impl From<Vec2> for cgmath::Vector2<f32> {
    fn from(v: Vec2) -> Self {
        v.0
    }
}

// Operators
impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self(self.0 * scalar)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.0 *= scalar;
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2(vec.0 * self)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self(self.0 / scalar)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.0 /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


pub fn reflect_vector_a_around_b(a: Vec2, b: Vec2) -> Vec2 {
    // 1. Find a vector perpendicular to b
    let b_perp = Vec2::new(-b.y, b.x); // Perpendicular vector

    // Handle case where b or b_perp are zero vectors
    if b_perp.magnitude2() == 0.0 || b.magnitude2() == 0.0 {
        return a; // Or handle as an error
    }

    // 2. Calculate the projection of a onto b_perp
    // projection = (a dot b_perp / b_perp dot b_perp) * b_perp
    let dot_a_b_perp = a.dot(b_perp);
    let dot_b_perp_b_perp = b_perp.dot(b_perp);
    let projection = b_perp * (dot_a_b_perp / dot_b_perp_b_perp);

    // 3. Apply the reflection formula: a_reflected = 2 * projection - a
    let a_reflected = 2.0 * projection - a;

    a_reflected
}

// pub fn fmt_vec_2(v: Vec2, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "({}, {})", v[0], v[1])
// }


// pub fn rotate_vector_deg(vec: Vec2, angle_deg: f32) -> Vec2 {
//     let rotation: Basis2<f32> = Rotation2::from_angle(Deg(angle_deg));
//     let rotated_vector = rotation.rotate_vector(vec.0);
//     Vec2(rotated_vector)
// }

// pub fn rotate_vector_rad(vec: Vec2, angle_rad: f32) -> Vec2 {
//     let rotation: Basis2<f32> = Rotation2::from_angle(Rad(angle_rad));
//     let rotated_vector = rotation.rotate_vector(vec.0);
//     Vec2(rotated_vector)
// }