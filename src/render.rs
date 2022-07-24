use crate::camera;
use crate::hit::hittable::Hittable;
use crate::math;
use crate::math::ray;
use crate::output;
use crate::scene;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Instant;
pub struct RayTracing {}
impl RayTracing {
    pub fn render(
        image_width: usize,
        image_height: usize,
        samples_per_pixels: usize,
        max_depth: usize,
        thread_count: usize,
        cam: Arc<camera::Camera>,
        world: Arc<scene::Scene>,
        output: Arc<Mutex<output::bitmap::Bitmap>>,
    ) {
        let mut thread_handles = vec![];
        let rows_per_thead = image_height / thread_count;
        for t in 0..thread_count {
            let cam_for_child = cam.clone();
            let world_for_child = world.clone();
            let output_for_child = output.clone();
            thread_handles.push(spawn(move || {
                let start_row = t * rows_per_thead;
                let end_row = if t + 1 == thread_count {
                    image_height
                } else {
                    start_row + rows_per_thead
                };
                let colors = RayTracing::render_rows(
                    t,
                    image_width,
                    image_height,
                    start_row,
                    end_row,
                    samples_per_pixels,
                    max_depth,
                    &cam_for_child,
                    &world_for_child,
                    &output_for_child,
                );
            }));
        }
        for handle in thread_handles {
            handle.join().unwrap();
        }
        println!("Threads finished");
    }
    fn render_rows(
        thread_index: usize,
        image_width: usize,
        image_height: usize,
        start_row: usize,
        end_row: usize,
        samples_per_pixels: usize,
        max_depth: usize,
        cam: &camera::Camera,
        world: &scene::Scene,
        output: &Mutex<output::bitmap::Bitmap>
    ) -> Vec<math::vector::Color3> {
        let mut colors: Vec<math::vector::Color3> = vec![];
        for h in start_row..end_row {
            let row_time = Instant::now();
            for w in 0..image_width {
                let mut color = math::vector::Color3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixels {
                    let rw: f64 = math::random::generate();
                    let rh: f64 = math::random::generate();
                    let u = (w as f64 + rw) / (image_width as f64 - 1.0);
                    let v = ((image_height - h - 1) as f64 + rh) / (image_height as f64 - 1.0);
                    let ray = cam.get_ray(u, v);
                    color += RayTracing::ray_color(&ray, &world, max_depth);
                }
                color /= samples_per_pixels as f64;
                output.lock().unwrap().write_color(w, h, &color);
            }
            println!(
                "Thread#{} Finished, Row time: {} secs",
                thread_index,
                row_time.elapsed().as_secs(),
            );
        }
        colors
    }
    fn ray_color(ray: &ray::Ray, world: &scene::Scene, depth: usize) -> math::vector::Color3 {
        if depth <= 0 {
            return math::vector::Color3::new(0.0, 0.0, 0.0);
        }
        let hit = world.hit(&ray, 0.0001, 10.0);
        match hit {
            Option::Some(rec) => {
                let scatter = rec.material().scatter(&ray, &rec);
                match scatter {
                    Option::Some(sr) => {
                        return RayTracing::ray_color(sr.ray(), &world, depth - 1)
                            * sr.attenuation();
                    }
                    Option::None => {
                        return math::vector::Color3::new(0.0, 0.0, 0.0);
                    }
                }
            }
            Option::None => {
                let unit = math::vector::Vec3::unit(&ray.dir());
                let t = (unit.y() + 1.0) * 0.5;
                return math::vector::Color3::new(0.5, 0.7, 1.0) * t
                    + math::vector::Color3::new(1.0, 1.0, 1.0) * (1.0 - t);
            }
        }
    }
}
