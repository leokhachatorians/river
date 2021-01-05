use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utility::{dot};
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }


        let t = root;
        let p = r.at(t);
        let outward_normal: Vec3 =  (p - self.center) / self.radius;
        let mut record = HitRecord{
            p: p,
            normal: outward_normal,
            t: t,
            front_face: false,
            material: &self.material
        };

        record.set_face_normal(r, outward_normal);

        return Some(record)
    }
}

