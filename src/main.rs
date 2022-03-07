use crate::hit::Hittable;
mod camera;
mod hit;
mod hittable_list;
mod output;
mod ray;
mod sphere;
mod math;
mod render;


fn main() {
    // Bitmap
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut img = output::bitmap::Bitmap::new(image_width, image_height);
    // Camera
    let cam = camera::Camera::new(aspect_ratio);
    // Scene
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        math::vector::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        math::vector::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    // Render
    render::RayTracing::render(image_width, image_height, 1, &cam, &world, &mut img);
    // Output
    let img_file = output::picture::Png::new("target/output.png".to_string());
    img_file.save(&img);
}
