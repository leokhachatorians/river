use crate::ray::{Ray};
use crate::utility::{
    degrees_to_radians, cross, unit_vector, random_double_range,
};
use crate::vec3::{Point3, Vec3, random_in_unit_disk};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32
}

impl Camera {
    pub fn new(
        look_from: Point3, look_at: Point3, vup: Vec3,
        vfov: f32, aspect_ratio: f32, aperture: f32,
        focus_dist: f32, _time0: f32, _time1: f32,
    ) -> Camera {
        let theta: f32 = degrees_to_radians(vfov);
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: u,
            v: v,
            lens_radius: lens_radius,
            time0: _time0,
            time1: _time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::new(
           self.origin + offset,
           self.lower_left_corner + s * self.horizontal + 
           t * self.vertical - self.origin - offset,
           random_double_range(self.time0, self.time1),
        )
    }
}
