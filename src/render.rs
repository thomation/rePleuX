use crate::camera;
use crate::hit;
use crate::hit::hittable::Hittable;
use crate::scene;
use crate::math;
use crate::output;
use crate::math::ray;

pub struct RayTracing {}
impl RayTracing {
    pub fn render(
        image_width: usize,
        image_height: usize,
        samples_per_pixels: usize,
        cam: &camera::Camera,
        world: &scene::Scene,
        output: &mut output::bitmap::Bitmap,
    ) {
        for h in 0..image_height {
            for w in 0..image_width {
                let mut color = math::vector::Color3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixels {
                    let rw: f64 = math::random::generate();
                    let rh: f64 = math::random::generate();
                    let u = (w as f64 + rw) / (image_width as f64 - 1.0);
                    let v = ((image_height - h - 1) as f64 + rh) / (image_height as f64 - 1.0);
                    let ray = cam.get_ray(u, v);
                    color += RayTracing::ray_color(&ray, &world, 50);
                }
                color /= samples_per_pixels as f64;
                output.write_color(w, h, &color);
            }
        }
    }
    fn ray_color(ray: &ray::Ray, world: &scene::Scene, depth: i8) -> math::vector::Color3 {
        if depth <= 0 {
            return math::vector::Color3::new(0.0, 0.0, 0.0);
        }
        let hit = world.hit(&ray, 0.1, 10.0);
        match hit {
            Option::Some(r) => {
                let target = r.position() + r.normal() + RayTracing::random_in_unit_sphere();
                let ray = math::ray::Ray::new(r.position(), target - r.position());
                return RayTracing::ray_color(&ray, &world, depth - 1) * 0.5;
            }
            Option::None => {
                let unit = math::vector::Vec3::unit(&ray.dir());
                let t = (unit.y() + 1.0) * 0.5;
                return math::vector::Color3::new(0.5, 0.7, 1.0) * t
                    + math::vector::Color3::new(1.0, 1.0, 1.0) * (1.0 - t);
            }
        }
    }
    fn random_in_unit_sphere() -> math::vector::Vec3 {
        loop {
            let p = math::vector::Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}
