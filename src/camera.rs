use crate::math::ray;
use crate::math::vector;

pub struct Camera {
    origin: vector::Point3,
    lower_left_corner_world: vector::Point3,
    horizontal: vector::Point3,
    vertical: vector::Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, vfov: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_lenght = 1.0;
        let origin = vector::Point3::new(0.0, 0.0, 0.0);
        let horizontal = vector::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vector::Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner_view = -horizontal.clone() / 2.0
            - vertical.clone() / 2.0
            - vector::Vec3::new(0.0, 0.0, focal_lenght);
        let lower_left_corner_world = origin.clone() + lower_left_corner_view;
        Camera {
            origin: origin,
            lower_left_corner_world: lower_left_corner_world,
            horizontal: horizontal,
            vertical: vertical,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        let p = self.lower_left_corner_world.clone() + self.horizontal.clone() * u + self.vertical.clone() * v;

        let dir = p - &self.origin;
        ray::Ray::new(self.origin.clone(), dir)
    }
}
