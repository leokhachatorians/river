use crate::ray::{Ray};
use crate::utility::{degrees_to_radians};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let focal_length: f64 = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 -
            vertical / 2.0 -
            Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
       Ray::new(
           self.origin,
           self.lower_left_corner + u * self.horizontal + 
           v * self.vertical - self.origin
        )
    }
}
