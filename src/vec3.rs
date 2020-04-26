use std::ops::{
    Add, AddAssign, Sub, SubAssign,
    Div, DivAssign, Mul, MulAssign
};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub elements: [f64; 3]
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {elements: [e0, e1, e2]}
    }

    pub fn x(&self) -> f64 {
        self.elements[0]
    }

    pub fn y(&self) -> f64 {
        self.elements[1]
    }

    pub fn z(&self) -> f64 {
        self.elements[2]
    }

    pub fn r(&self) -> f64 {
        self.elements[0]
    }

    pub fn g(&self) -> f64 {
        self.elements[1]
    }

    pub fn b(&self) -> f64 {
        self.elements[2]
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

//impl Div<Vec3> for Vec3 {
//    type Output = Self;
//
//    fn div(self, other: Scalar) {
//        
//    }
//}


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

}
