use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use crate::hit::Hittable;
mod ray;
mod vector;
mod hit;
mod sphere;
mod hittable_list;
mod camera;

fn ray_color(ray: &ray::Ray, hit: Option<hit::HitRecord>) -> vector::Color3 {
    match hit {
        Option::Some(r) => {
            return vector::Color3::new(r.normal().x() + 1.0, r.normal().y() + 1.0, r.normal().z() + 1.0) * 0.5;
        },
        Option::None => {
            let unit = vector::Vec3::unit(&ray.dir());
            let t = (unit.y() + 1.0) * 0.5;
            return vector::Color3::new(0.5, 0.7, 1.0) * t + vector::Color3::new(1.0, 1.0, 1.0) * (1.0 - t)
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let cam = camera::Camera::new(aspect_ratio);
    // Scene
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(vector::Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(sphere::Sphere::new(vector::Point3::new(0.0, -100.5, -1.0), 100.0)));
    // Render
    let channel = 3;
    let mut pixels = vec![0; image_width * image_height * channel];
    for h in 0..image_height {
        for w in 0..image_width {
            // Screen Coordinate
            let u = w as f64 / (image_width as f64 - 1.0);
            let v = (image_height - h - 1) as f64 / (image_height as f64 - 1.0);
            // World Coordinate
            let ray = cam.get_ray(u, v);
            let hit = world.hit(&ray, 0.1, 10.0);
            let color = ray_color(&ray, hit);
            pixels[w * channel + h * image_width * channel] = (color.x() * 255.999) as u8;
            pixels[1 + w * channel + h * image_width * channel] = (color.y() * 255.999) as u8;
            pixels[2 + w * channel + h * image_width * channel] = (color.z() * 255.999) as u8;
        }
    }
    let output = File::create("target/output.png").expect("cannot create output.png");
    let encorder = PNGEncoder::new(output);
    encorder
        .encode(
            &pixels,
            image_width as u32,
            image_height as u32,
            ColorType::RGB(8),
        )
        .expect("encode png fail");
}
