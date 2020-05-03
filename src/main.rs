mod ray;
mod utility;
mod vec3;

const IMAGE_WIDTH: i32 = 200;
const IMAGE_HEIGHT: i32 = 100;

fn ray_color(r: &ray::Ray) -> vec3::Vec3 {
    let v = vec3::Vec3::new(0.0, 0.0, -1.0);
    let t =  hit_sphere(v, 0.5, r);
    if t > 0.0 {
        let subtracted_by = vec3::Vec3::new(0.0, 0.0, -1.0);
        let N = utility::unit_vector(r.at(t) - subtracted_by);
        return 0.5 * vec3::Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    let unit_direciton = utility::unit_vector(r.direction());
    let t = 0.5 * (unit_direciton.y() + 1.0);
    return (1.0 - t) * vec3::Vec3::new(1.0, 1.0, 1.0) + t * vec3::Vec3::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: vec3::Vec3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = utility::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::Vec3::new(0.0, 0.0, 0.0);

    for j in (0..IMAGE_HEIGHT).rev() {
        //println!("\rScanlines Remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / IMAGE_WIDTH as f64;
            let v = j as f64 / IMAGE_HEIGHT as f64;
            let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let color = ray_color(&r);
            color.write_color();
            //let color = vec3::Vec3::new(
            //    i as f64  / IMAGE_WIDTH as f64,
            //    j as f64 / IMAGE_HEIGHT as f64,
            //    0.2
            //);
            //color.write_color();

            //let r = i as f32 / IMAGE_WIDTH as f32;
            //let g = j as f32 / IMAGE_HEIGHT as f32;

            //let b = 0.2;

            //let ir: i32 = (255.999 * r) as i32;
            //let ig: i32 = (255.999 * g) as i32;
            //let ib: i32 = (255.999 * b) as i32;

            //println!("{} {} {}", ir, ig, ib);
        }
    }

    let v1 = vec3::Vec3::new(1.0, 2.0, 3.0);
    let v2 = vec3::Vec3::new(1.0, 2.0, 3.0);
    let r = ray::Ray::new(v1, v2);
    
}
