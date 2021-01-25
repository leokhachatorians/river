use crate::ray::Ray;
use crate::vec3::{Point3};

pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        AABB { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction()[i];
            let t0 = (self.min()[i] - ray.origin()[i]) * inv_d;
            let t1 = (self.max()[i] - ray.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                let (t0, t1) = (t1, t0);
            }

            let check_min = if t0 > t_min { t0 } else { t_min };
            let check_max = if t1 < t_max { t1 } else { t_max };

            if check_max <= check_min {
                return false;
            }
        }
        true
    }
}
