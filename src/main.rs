use std::fs::File;
use image::ColorType;
use image::png::PNGEncoder;
mod vector;
mod ray;

fn ray_color(ray: &ray::Ray) -> vector::Color3
{
    let unit = vector::Vec3::unit(&ray.dir());
    let t = (unit.y() + 1.0) * 0.5;
    vector::Color3::new(0.5, 0.7, 1.0) * t + vector::Color3::new(1.0, 1.0, 1.0) * (1.0 - t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_lenght = 1.0;
    let origin = vector::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vector::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vector::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone() - horizontal.clone() / 2.0 - vertical.clone() / 2.0 - vector::Vec3::new(0.0, 0.0, focal_lenght);

    // Render
    let channel = 3;
    let mut pixels = vec![0; image_width * image_height * channel];
    for h in 0..image_height{
        for w in 0..image_width {
            let u = w as f64 / (image_width as f64 - 1.0);
            let v = (image_height - h - 1) as f64 / (image_height as f64 - 1.0);
            let ray = ray::Ray::new(origin.clone(), lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v - origin.clone());
            let color = ray_color(&ray);
            pixels[w * channel + h * image_width * channel] = (color.x() * 255.999) as u8;
            pixels[1 + w * channel + h * image_width * channel] = (color.y() * 255.999) as u8;
            pixels[2 + w * channel + h * image_width * channel] = (color.z() * 255.999) as u8;
        }
    }
    let output = File::create("target/output.png").expect("cannot create output.png");
    let encorder = PNGEncoder::new(output);
    encorder.encode(&pixels, image_width as u32, image_height as u32, ColorType::RGB(8)).expect("encode png fail");
}
