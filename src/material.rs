use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utility::{random_double};
use crate::vec3::{
    Color, Vec3,
    dot, random_unit_vector, reflect,
    unit_vector, random_unit_in_sphere,
    refract
};

#[derive(Copy, Clone)]
pub enum Material {
    Metal {
        albedo: Color,
        fuzz: f32
    },

    Lambertian {
        albedo: Color
    },

    Dielectric {
        index_of_refraction: f32
    }
}

impl Material {
    pub fn scatter(self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, bool)> {
        match self {
            Material::Metal { albedo, fuzz } => {
                let reflected: Vec3 = reflect(unit_vector(r_in.direction()), rec.normal);
                let scattered = Ray::new(rec.p, reflected + fuzz * random_unit_in_sphere(), r_in.time());
                let attenuation = albedo;
                Some((scattered, attenuation, dot(scattered.direction(), rec.normal) > 0.0))
            }

            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
                let attenuation = albedo;
                Some((scattered, attenuation, true))
            }

            Material::Dielectric { index_of_refraction } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face {
                    1.0 / index_of_refraction
                } else {
                    index_of_refraction
                };

                let unit_direction = unit_vector(r_in.direction());

                let dot_product = dot(-unit_direction, rec.normal);
                let cos_theta: f32 = if dot_product < 1.0 {
                    dot_product
                } else {
                    1.0
                };

                let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
                let direction: Vec3;

                if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                    direction = reflect(unit_direction, rec.normal);
                } else {
                    direction = refract(unit_direction, rec.normal, refraction_ratio);
                }

                let scattered = Ray::new(rec.p, direction, r_in.time());
                Some((scattered, attenuation, true))
            }
        }
    }
}


fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // use Schlick's Approximation
    let mut r0: f32 = (1.0 - ref_idx)  / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
