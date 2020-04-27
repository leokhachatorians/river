mod utility;
mod vec3;

const IMAGE_WIDTH: i32 = 200;
const IMAGE_HEIGHT: i32 = 100;

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let color = vec3::Vec3::new(
                i as f64  / IMAGE_WIDTH as f64,
                j as f64 / IMAGE_HEIGHT as f64,
                0.2
            );
            color.write_color();
            //let r = i as f32 / IMAGE_WIDTH as f32;
            //let g = j as f32 / IMAGE_HEIGHT as f32;

            //let b = 0.2;

            //let ir: i32 = (255.999 * r) as i32;
            //let ig: i32 = (255.999 * g) as i32;
            //let ib: i32 = (255.999 * b) as i32;

            //println!("{} {} {}", ir, ig, ib);
        }
    }
    
}
