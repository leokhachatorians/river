use std::ops;

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

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z()
            ]
        }
    }
}
