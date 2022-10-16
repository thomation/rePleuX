mod hit;
mod material;
mod math;
mod object;
mod io;
mod render;
mod scene;
mod texture;

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

fn main() {
    // let now = Instant::now();
    // let samples_per_pixel = 128;
    // let max_depth = 50;

    // let world = scene::scene::Scene::new();
    // let aspect_ratio = world.camera().aspect_ratio();
    // let image_width = 800;
    // let image_height = (image_width as f64 / aspect_ratio) as usize;
    // let img = Arc::new(Mutex::new(io::bitmap::Bitmap::new(image_width, image_height)));
    // render::RayTracing::render2(
    //     image_width,
    //     image_height,
    //     samples_per_pixel,
    //     max_depth,
    //     6,
    //     Arc::new(world),
    //     img.clone()
    // );
    // let img_file = io::picture::Png::new("target/output.png".to_string());
    // img_file.save(img);
    // println!("Run time: {} secs", now.elapsed().as_secs());
    

    // Test mc
    const N:usize = 10000;
    let mut inside_circle = 0;
    for _ in 0..N {
        let x = math::random::generate_range(-1.0, 1.0);
        let y = math::random::generate_range(-1.0, 1.0);
        if x * x + y* y < 1.0 {
            inside_circle += 1;
        }
    }
    println!("Estimate PI: {}", 4.0 * inside_circle as f64 / N as f64);

}
