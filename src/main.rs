mod camera;
mod output;
mod math;
mod render;
mod scene;
mod object;
mod hit;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let mut img = output::bitmap::Bitmap::new(image_width, image_height);
    let cam = camera::Camera::new(aspect_ratio);
    let world = scene::Scene::new();
    render::RayTracing::render(image_width, image_height, 4, &cam, &world, &mut img);
    let img_file = output::picture::Png::new("target/output.png".to_string());
    img_file.save(&img);
}
