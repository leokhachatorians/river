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
use crate::sphere::{Sphere, MovingSphere};
use crate::utility::{
    INFINITY, unit_vector,
    random_double, clamp,
    random_double_range,
};
use crate::vec3::{Vec3, Color, Point3};

use rayon::prelude::*;
use std::fs;


fn ray_color(ray: ray::Ray, world: &hittable::HittableList, depth: usize) -> vec3::Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        if let Some(scatter_tuple) = hit.material.scatter(&ray, &hit) {
            let (scattered, attenuation, hit) = scatter_tuple;

            if hit {
                return attenuation * ray_color(scattered, world, depth -1 );
            }
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direciton = unit_vector(ray.direction());
    let t = 0.5 * (unit_direciton.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn scene() -> HittableList {
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5)
    };

    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    objects.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));

    for i in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center: Point3 = Point3::new(
                i as f32 + 0.9*random_double(),
                0.2,
                b as f32 + 0.9*random_double()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let albedo: Color;
                let fuzz: f32;

                if choose_mat < 0.8 {
                    // diffuse
                    albedo = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian { albedo };
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);

                    objects.push(Box::new(
                        MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material)
                    ));
                }
                else if choose_mat < 0.95 {
                    // metal
                    albedo = Color::random_range(0.5, 1.0);
                    fuzz = random_double();
                    let sphere_material = Material::Metal{ albedo, fuzz };
                    objects.push(Box::new(
                        Sphere::new(center, 0.2, sphere_material)
                    ));
                }
                else {
                    // glass
                    let sphere_material = Material::Dielectric { index_of_refraction: 1.5 };
                    objects.push(Box::new(
                        Sphere::new(center, 0.2, sphere_material)
                    ));
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

    objects.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1)));
    objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2)));
    objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3)));

    HittableList::new(objects)
}

fn balls_on_plain(iterations: usize) {
    let world = scene();

    let mut x = 13.0;
    let mut z = 3.0;

    let x_increment = 0.05;
    let z_increment = 0.1;

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = ((image_width as f32) / aspect_ratio) as usize;
    let samples_per_pizel: usize = 100;
    let max_depth: usize = 50;

    for iteration in 1..iterations + 1 {
        println!("Starting iteration: {}", iteration);
        let look_from: Point3 = Point3::new(x, 2.0, z);
        x += x_increment;
        z += z_increment;
        let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
        let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = 10.0;
        let aperture: f32 = 0.1;

        let camera = Camera::new(
            look_from, look_at, vup,
            20.0, aspect_ratio, aperture, dist_to_focus,
            0.0, 1.0
        );

        let pixels = (0..image_height)
            .into_par_iter()
            .rev()
            .map(|j| {
                (0..image_width)
                    .into_par_iter()
                    .map(|i| {
                        let mut col = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pizel {
                            let u = (i as f32 + random_double()) / (image_width as f32 - 1.0);
                            let v = (j as f32 + random_double()) / (image_height as f32 - 1.0);
                            let ray = camera.get_ray(u, v);
                            col += ray_color(ray, &world, max_depth);
                        }
                        col = col / samples_per_pizel as f32;
                        col = Color::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
                        let ir = 255.99 * clamp(col.x(), 0.0, 0.999);
                        let ig = 255.99 * clamp(col.y(), 0.0, 0.999);
                        let ib = 255.99 * clamp(col.z(), 0.0, 0.999);
                        format!("{} {} {}\n", ir as i32, ig as i32, ib as i32)
                    })
                .collect::<Vec<String>>()
                .join("")
            })
        .collect::<Vec<String>>()
        .join("");

        let mut pic = format!("P3\n{} {}\n255\n", image_width, image_height);
        pic = format!("{}{}", &pic, pixels);

        let file_name = format!("output-{}.ppm", iteration);

        println!("Writing iteration: {}", file_name);
        if fs::write(file_name, pic).is_err() {
            eprintln!("Error generating image");
        };
    }
}


fn main() {
    balls_on_plain(1);
}
