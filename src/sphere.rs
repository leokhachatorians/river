use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utility::{dot};
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
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
        let p = ray.at(t);
        let outward_normal: Vec3 =  (p - self.center) / self.radius;
        let mut record = HitRecord{
            p: p,
            normal: outward_normal,
            t: t,
            front_face: false,
            material: &self.material
        };

        record.set_face_normal(ray, outward_normal);

        return Some(record)
    }
}

