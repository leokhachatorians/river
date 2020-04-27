use crate::vec3::Vec3;

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.x() * v2.x() +
        v1.y() * v2.y() +
        v1.z() * v2.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x()
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 5.0, 7.0);

        assert_eq!(dot(v1, v2), 32.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 5.0, 7.0);

        let product = Vec3::new(-1.0, -4.0, 3.0);

        assert_eq!(cross(v1, v2), product);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(2.0, -4.0, 1.0);
        let result = Vec3::new(
            0.4364357804719848,
            -0.8728715609439696,
            0.2182178902359924
        );

        assert_eq!(unit_vector(v), result);
    }
}
