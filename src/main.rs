mod camera;
mod output;
mod math;
mod render;
mod scene;

fn main() {
    // Bitmap
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut img = output::bitmap::Bitmap::new(image_width, image_height);
    // Camera
    let cam = camera::Camera::new(aspect_ratio);
    // Scene
    let world = scene::world::World::new();
    // Render
    render::RayTracing::render(image_width, image_height, 1, &cam, &world, &mut img);
    // Output
    let img_file = output::picture::Png::new("target/output.png".to_string());
    img_file.save(&img);
}
