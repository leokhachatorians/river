use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{
    Color, Vec3,
    dot, random_unit_vector, reflect,
    unit_vector, random_unit_in_sphere,
    refract
};

pub trait Material {
    fn scatter(
        &self, r_in: &Ray, rec: &HitRecord,
        //attenuation: &mut Color, scattered: &mut Ray
    ) -> Option<(Ray, Color, bool)>;
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

pub struct Lambertian {
    albedo: Color
}

pub struct Dielectric {
    index_of_refraction: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo: albedo, fuzz: fuzz }
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo: albedo }
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric { index_of_refraction: index_of_refraction }
    }
}

impl Material for Metal {
    fn scatter<'a> (
        &'a self, r_in: &Ray, hit: &HitRecord,
    ) -> Option<(Ray, Color, bool)> {
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), hit.normal);
        let mut scattered = Ray::new(hit.p, reflected + self.fuzz * random_unit_in_sphere());
        let mut attenuation = self.albedo;
        Some((scattered, attenuation, dot(scattered.direction(), hit.normal) > 0.0))
    }
}

impl Material for Lambertian {
    fn scatter<'a> (
        &'a self, r_in: &Ray, hit: &HitRecord,
    ) -> Option<(Ray, Color, bool)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let mut scattered = Ray::new(hit.p, scatter_direction);
        let mut attenuation = self.albedo;
        Some((scattered, attenuation, true))
    }
}

impl Material for Dielectric {
    fn scatter<'a>(&'a self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color, bool)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = unit_vector(r_in.direction());

        let dot_product = dot(-unit_direction, hit.normal);
        let cos_theta: f64 = if dot_product < 1.0 {
            dot_product
        } else {
            1.0
        };

        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract {
            direction = reflect(unit_direction, hit.normal);
        } else {
            direction = refract(unit_direction, hit.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit.p, direction);
        Some((scattered, attenuation, true))
    }
}
