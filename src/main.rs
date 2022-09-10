mod camera;
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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 16;
    let max_depth = 20;

    let img = Arc::new(Mutex::new(output::bitmap::Bitmap::new(image_width, image_height)));
    let look_from = math::vector::Point3::new(13.0, 2.0, 3.0);
    let look_at = math::vector::Point3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let cam = camera::Camera::new(
        look_from,
        look_at,
        math::vector::Dir3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        focus_dist,
        0.0,
        1.0,
    );
    let world = scene::scene::Scene::new();
    render::RayTracing::render(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        6,
        Arc::new(cam),
        Arc::new(world),
        img.clone()
    );
    let img_file = output::picture::Png::new("target/output.png".to_string());
    img_file.save(img);
    println!("Run time: {} secs", now.elapsed().as_secs());
}
