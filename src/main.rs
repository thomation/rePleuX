mod hit;
mod material;
mod math;
mod object;
mod output;
mod render;
mod scene;
mod texture;

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let samples_per_pixel = 128;
    let max_depth = 50;

    let world = scene::scene::Scene::new();
    let aspect_ratio = world.camera().aspect_ratio();
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let img = Arc::new(Mutex::new(output::bitmap::Bitmap::new(image_width, image_height)));
    render::RayTracing::render(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        6,
        Arc::new(world),
        img.clone()
    );
    let img_file = output::picture::Png::new("target/output.png".to_string());
    img_file.save(img);
    println!("Run time: {} secs", now.elapsed().as_secs());
}
