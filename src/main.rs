mod camera;
mod ray;
mod utility;
mod vec3;
mod sphere;
mod hittable;
mod material;

use crate::camera::{Camera};
use crate::hittable::{Hittable, HittableList};
use crate::material::{Material};
use crate::sphere::{Sphere};
use crate::utility::{
    INFINITY, unit_vector,
    random_double, clamp,
};
use crate::vec3::{Vec3, Color, Point3};

const IMAGE_WIDTH: i32 = 200;
const ASPECT_RATIO: f32 = 3.0 / 2.0;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: &ray::Ray, world: &hittable::HittableList, depth: i32) -> vec3::Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, 0.001, INFINITY) {
        // Real?

        if let Some(scatter_tuple) = hit.material.scatter(&r, &hit) {
            let (scattered, attenuation, hit) = scatter_tuple;

            if hit {
                return attenuation * ray_color(&scattered, world, depth -1 );
            }
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direciton = unit_vector(r.direction());
    let t = 0.5 * (unit_direciton.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn write_color(color: Color, samples_per_pixel: f32) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / samples_per_pixel;

    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!(
        "{:?} {:?} {:?}",
        (255.99 * clamp(r, 0.0, 0.999)) as i32,
        (255.99 * clamp(g, 0.0, 0.999)) as i32,
        (255.99 * clamp(b, 0.0, 0.999)) as i32,
    );
}

fn scene() -> HittableList {
    let mut world: hittable::HittableList = Default::default();
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5)
    };

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for i in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center: Point3 = Point3::new(
                i as f32 + 0.9*random_double(),
                0.2,
                b as f32 + 0.9*random_double()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                //let mut sphere_material;
                let albedo: Color;
                let fuzz: f32;

                if choose_mat < 0.8 {
                    // diffuse
                    albedo = Color::random() * Color::random();
                    //sphere_material = Lambertian::new(albedo);
                    world.add(
                        Sphere::new(center, 0.2, Material::Lambertian {
                            albedo: albedo
                        })
                    );
                }
                else if choose_mat < 0.95 {
                    // metal
                    albedo = Color::random_range(0.5, 1.0);
                    fuzz = random_double();
                    //sphere_material = Metal::new(albedo, fuzz);
                    world.add(
                        Sphere::new(center, 0.2, Material::Metal {
                            albedo: albedo, fuzz: fuzz
                        })
                    );
                }
                else {
                    // glass
                    //sphere_material = Dielectric::new(1.5);
                    world.add(
                        Sphere::new(center, 0.2, Material::Dielectric {
                            index_of_refraction: 1.5
                        })
                    );
                }
            }
        }
    }

    let material_1 = Material::Dielectric {
        index_of_refraction: 1.5
    };
    let material_2 = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1)
    };
    let material_3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0
    };

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3));

    world
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // World
    let world = scene();

    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.1;

    let camera = Camera::new(
        look_from, look_at, vup,
        20.0, ASPECT_RATIO, aperture, dist_to_focus
    );

    // Render

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_double()) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + random_double()) / (IMAGE_HEIGHT as f32 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as f32)
        }
    }
}
