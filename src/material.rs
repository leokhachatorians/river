use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{
    Color, Vec3,
    dot, random_unit_vector, reflect,
    unit_vector
};

pub trait Material {
    fn scatter(
        &self, r_in: &Ray, rec: &HitRecord,
        //attenuation: &mut Color, scattered: &mut Ray
    ) -> Option<(Ray, Color, bool)>;
}

pub struct Metal {
    pub albedo: Color,
    //pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo: albedo }
    }
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter<'a> (
        &'a self, r_in: &Ray, hit: &HitRecord,
        //attenuation: &'a mut Color, scattered: &mut Ray
    ) -> Option<(Ray, Color, bool)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let mut scattered = Ray::new(hit.p, scatter_direction);
        let mut attenuation = self.albedo;
        Some((scattered, attenuation, true))
        //return true;
    }
}

impl Material for Metal {
    fn scatter<'a> (
        &'a self, r_in: &Ray, hit: &HitRecord,
        //attenuation: &'a mut Color, scattered: &mut Ray
    ) -> Option<(Ray, Color, bool)> {
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), hit.normal);
        let mut scattered = Ray::new(hit.p, reflected);
        let mut attenuation = self.albedo;
        Some((scattered, attenuation, dot(scattered.direction(), hit.normal) > 0.0))
    }

}
