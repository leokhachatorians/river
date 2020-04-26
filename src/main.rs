const IMAGE_WIDTH: i32 = 200;
const IMAGE_HEIGHT: i32 = 100;

mod vec3;

fn main() {

    //println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    //for j in (0..IMAGE_HEIGHT).rev() {
    //    for i in 0..IMAGE_WIDTH {
    //        let r = i as f32 / IMAGE_WIDTH as f32;
    //        let g = j as f32 / IMAGE_HEIGHT as f32;

    //        let b = 0.2;

    //        let ir: i32 = (255.999 * r) as i32;
    //        let ig: i32 = (255.999 * g) as i32;
    //        let ib: i32 = (255.999 * b) as i32;

    //        println!("{} {} {}", ir, ig, ib);
    //    }
    //}
    //
    let mut v = vec3::Vec3::new(1.0, 2.0, 3.0);
    let other = vec3::Vec3::new(1.0, 2.0, 3.0);

    v -= other;

    println!("X: {}", v.x());
    println!("Y: {}", v.y());
    println!("Z: {}", v.z());
    println!("R: {}", v.r());
    println!("G: {}", v.g());
    println!("B: {}", v.b());
}
