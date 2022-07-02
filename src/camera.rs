use crate::math::ray;
use crate::math::vector;

pub struct Camera {
    origin: vector::Point3,
    lower_left_corner_world: vector::Point3,
    horizontal: vector::Point3,
    vertical: vector::Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_lenght = 1.0;
        let origin = vector::Point3::new(0.0, 0.0, 0.0);
        let horizontal = vector::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vector::Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner_view = -horizontal/ 2.0
            - vertical/ 2.0
            - vector::Vec3::new(0.0, 0.0, focal_lenght);
        let lower_left_corner_world = origin+ lower_left_corner_view;
        Camera {
            origin: origin,
            lower_left_corner_world: lower_left_corner_world,
            horizontal: horizontal,
            vertical: vertical,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        let p = self.lower_left_corner_world
            + self.horizontal* u
            + self.vertical* v;

        let dir = p - self.origin;
        ray::Ray::new(self.origin, dir)
    }
}
