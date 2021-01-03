use crate::ray::{Ray};
use crate::utility::{
    degrees_to_radians, cross, unit_vector
};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point3,
}

impl Camera {
    pub fn new(
        look_from: Point3, look_at: Point3, vup: Vec3,
        vfov: f64, aspect_ratio: f64
    ) -> Camera {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 -w;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
       Ray::new(
           self.origin,
           self.lower_left_corner + s * self.horizontal + 
           t * self.vertical - self.origin
        )
    }
}
