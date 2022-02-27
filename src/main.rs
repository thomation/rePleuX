use crate::hit::Hittable;
mod ray;
mod vector;
mod hit;
mod sphere;
mod hittable_list;
mod camera;
mod output;

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
    let mut img = output::Image::new(aspect_ratio);
    let img_file = output::ImageFile::new("target/output.png".to_string());
    // Camera
    let cam = camera::Camera::new(aspect_ratio);
    // Scene
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(vector::Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(sphere::Sphere::new(vector::Point3::new(0.0, -100.5, -1.0), 100.0)));
    // Render
    let image_width = img.width();
    let image_height = img.height();
    for h in 0..image_height {
        for w in 0..image_width {
            // Screen Coordinate
            let u = w as f64 / (image_width as f64 - 1.0);
            let v = (image_height - h - 1) as f64 / (image_height as f64 - 1.0);
            let ray = cam.get_ray(u, v);
            let hit = world.hit(&ray, 0.1, 10.0);
            let color = ray_color(&ray, hit);
            img.write_color(w, h, &color);
        }
    }
    img_file.save(&img);
}
