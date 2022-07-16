use crate::math::ray;
use crate::math::vector;

pub struct Camera {
    origin: vector::Point3,
    lower_left_corner_world: vector::Point3,
    horizontal: vector::Point3,
    vertical: vector::Point3,
}

impl Camera {
    pub fn new(look_from: vector::Point3, look_at: vector::Point3, vup: vector::Dir3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let focal_lenght = 1.0;
        let h = (theta * 0.5).tan() * focal_lenght;
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let mut w = look_from.clone() - look_at;
        w.normalize();
        let mut u = vector::Vec3::cross(&vup, &w);
        u.normalize();
        let v = vector::Vec3::cross(&w, &u);
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        let origin = look_from;
        let lower_left_corner_view = -horizontal.clone() / 2.0
            - vertical.clone() / 2.0
            - w * focal_lenght;
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
