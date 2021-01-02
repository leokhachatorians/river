mod camera;
mod ray;
mod utility;
mod vec3;
mod sphere;
mod hittable;
mod material;

use crate::camera::{Camera};
use crate::hittable::Hittable;
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::utility::{
    INFINITY, unit_vector, dot,
    random_double, random_double_range, clamp,
};
use crate::vec3::{Vec3, Color};

const IMAGE_WIDTH: i32 = 200;
const IMAGE_HEIGHT: i32 = 100;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: &ray::Ray, world: &hittable::HittableList, depth: i32) -> vec3::Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, 0.001, INFINITY) {
        // Fake Lambertian diffuse
        //let target = hit.p + hit.normal + vec3::random_unit_vector();
        // Real?
        let target = hit.p + vec3::random_in_hemisphere(hit.normal);
        return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1)
        //#return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direciton = unit_vector(r.direction());
    let t = 0.5 * (unit_direciton.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: vec3::Vec3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn write_color(color: Color, samples_per_pixel: f64) {
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

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut world: hittable::HittableList = Default::default();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    for j in (0..IMAGE_HEIGHT).rev() {
        //println!("\rScanlines Remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            //println!("{:?}", pixel_color);

            write_color(pixel_color, SAMPLES_PER_PIXEL as f64)

            //let u = i as f64 / IMAGE_WIDTH as f64;
            //let v = j as f64 / IMAGE_HEIGHT as f64;
            //let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            //let color = ray_color(&r, &world);
            //color.write_color();
        }
    }
}
