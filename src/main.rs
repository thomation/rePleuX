use crate::hit::Hittable;
use rand::Rng;
mod camera;
mod hit;
mod hittable_list;
mod output;
mod ray;
mod sphere;
mod vector;

fn ray_color(ray: &ray::Ray, hit: Option<hit::HitRecord>) -> vector::Color3 {
    match hit {
        Option::Some(r) => {
            return vector::Color3::new(
                r.normal().x() + 1.0,
                r.normal().y() + 1.0,
                r.normal().z() + 1.0,
            ) * 0.5;
        }
        Option::None => {
            let unit = vector::Vec3::unit(&ray.dir());
            let t = (unit.y() + 1.0) * 0.5;
            return vector::Color3::new(0.5, 0.7, 1.0) * t
                + vector::Color3::new(1.0, 1.0, 1.0) * (1.0 - t);
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut img = output::bitmap::Bitmap::new(image_width, image_height);
    let img_file = output::picture::Png::new("target/output.png".to_string());
    // Camera
    let cam = camera::Camera::new(aspect_ratio);
    // Scene
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        vector::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vector::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    // Render
    let samples_per_pixels = 10;
    let mut rng = rand::thread_rng();
    for h in 0..image_height {
        for w in 0..image_width {
            let mut color = vector::Color3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixels {
                let rw: f64 = rng.gen();
                let rh: f64 = rng.gen();
                let u = (w as f64 + rw) / (image_width as f64 - 1.0);
                let v = ((image_height - h - 1) as f64 + rh) / (image_height as f64 - 1.0);
                let ray = cam.get_ray(u, v);
                let hit = world.hit(&ray, 0.1, 10.0);
                color += ray_color(&ray, hit);
            }
            color /= samples_per_pixels as f64;
            img.write_color(w, h, &color);
        }
    }
    img_file.save(&img);
}
