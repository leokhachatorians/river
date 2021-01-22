use std::ops::{
    Add, AddAssign, Sub, SubAssign,
    Div, DivAssign, Mul, MulAssign,
    Neg
};

use crate::utility::{random_double, random_double_range};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub elements: [f32; 3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {elements: [e0, e1, e2]}
    }

    pub fn x(&self) -> f32 {
        self.elements[0]
    }

    pub fn y(&self) -> f32 {
        self.elements[1]
    }

    pub fn z(&self) -> f32 {
        self.elements[2]
    }

    pub fn r(&self) -> f32 {
        self.elements[0]
    }

    pub fn g(&self) -> f32 {
        self.elements[1]
    }

    pub fn b(&self) -> f32 {
        self.elements[2]
    }

    pub fn length(&self) -> f32 {
        (self.length_squared()).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.99 * self.r()) as i32,
            (255.99 * self.g()) as i32,
            (255.99 * self.b()) as i32,
        );
    }

    pub fn random() -> Vec3 {
        Vec3 {
            elements: [random_double(), random_double(), random_double()]
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            elements: [random_double_range(min, max), random_double_range(min, max), random_double_range(min, max)]
        }
    }

    pub fn near_zero(&self) -> bool {
        let s: f32 = 1e-8;

        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x() * v2.x() +
        v1.y() * v2.y() +
        v1.z() * v2.z()
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_in_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_unit_in_sphere())
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let dot_product = dot(-uv, n);
    let cos_theta = if dot_product < 1.0 {
        dot_product
    } else {
        1.0
    };
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Vec3 {
        Self {
            elements: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z()
            ]
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            elements: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ]
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            elements: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z()
            ]
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            elements: [
                -self.x(),
                -self.y(),
                -self.z(),
            ]
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            elements: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ]
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            elements: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z()
            ]
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            elements: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, value: f32) -> Self {
        Self {
            elements: [
                self.x() * value,
                self.y() * value,
                self.z() * value
            ]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Vec3 {
        vec3 * self
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            elements: [
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, value: f32) -> Self {
        (1.0 / value) * self
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, vec3: Vec3) -> Vec3 {
        vec3 / self
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            elements: [
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_creation() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        
        assert_eq!(test_vec.x(), 1.0);
        assert_eq!(test_vec.y(), 2.0);
        assert_eq!(test_vec.z(), 3.0);
        assert_eq!(test_vec.r(), 1.0);
        assert_eq!(test_vec.g(), 2.0);
        assert_eq!(test_vec.b(), 3.0);
    }

    #[test]
    fn vec3_add() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(3.0, 4.0, 5.0);
        let sum = Vec3::new(4.0, 6.0, 8.0);

        assert_eq!(test_vec + other, sum);
    }

    #[test]
    fn vec3_add_assign() {
        let mut test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(3.0, 4.0, 5.0);
        let sum = Vec3::new(4.0, 6.0, 8.0);

        test_vec += other;

        assert_eq!(test_vec, sum);
    }

    #[test]
    fn vec3_sub() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(2.5, 1.0, 6.0);
        let diff = Vec3::new(-1.5, 1.0, -3.0);

        assert_eq!(test_vec - other, diff);
    }

    #[test]
    fn vec3_sub_assign() {
        let mut test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(2.5, 1.0, 6.0);
        let diff = Vec3::new(-1.5, 1.0, -3.0);

        test_vec -= other;

        assert_eq!(test_vec, diff);
    }

    #[test]
    fn vec3_mul() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(2.0, 2.0, 2.0);
        let product = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(test_vec * other, product);
    }

    #[test]
    fn vec3_mul_assign() {
        let mut test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(2.0, 2.0, 2.0);
        let product = Vec3::new(2.0, 4.0, 6.0);

        test_vec *= other;

        assert_eq!(test_vec, product);
    }

    #[test]
    fn vec3_mul_value_1() {
        let mut test_vec = Vec3::new(1.0, 2.0, 3.0);
        let value: f32 = 2.0;
        let product = Vec3::new(2.0, 4.0, 6.0);
        
        assert_eq!(test_vec * value, product);
    }

    #[test]
    fn vec3_mul_value_2() {
        let mut test_vec = Vec3::new(1.0, 2.0, 3.0);
        let value: f32 = 2.0;
        let product = Vec3::new(2.0, 4.0, 6.0);
        
        assert_eq!(value * test_vec, product);
    }

    #[test]
    fn vec3_div() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(2.0, 2.0, 2.0);
        let quotient = Vec3::new(0.5, 1.0, 1.5);

        assert_eq!(test_vec / other, quotient);
    }

    #[test]
    fn vec3_div_value_1() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        let value: f32 = 2.0;
        let quotient = Vec3::new(0.5, 1.0, 1.5);

        assert_eq!(test_vec / value, quotient);
    }

    #[test]
    fn vec3_div_value_2() {
        let test_vec = Vec3::new(1.0, 2.0, 3.0);
        let value: f32 = 2.0;
        let quotient = Vec3::new(0.5, 1.0, 1.5);

        assert_eq!(value / test_vec, quotient);
    }

    #[test]
    fn vec3_div_assign() {
        let mut test_vec = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(2.0, 2.0, 2.0);
        let quotient = Vec3::new(0.5, 1.0, 1.5);

        test_vec /= other;

        assert_eq!(test_vec, quotient);
    }
}
