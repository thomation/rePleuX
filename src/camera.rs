use crate::math::random;
use crate::math::ray;
use crate::math::vector;

pub struct Camera {
    origin: vector::Point3,
    lower_left_corner_world: vector::Point3,
    horizontal: vector::Point3,
    vertical: vector::Point3,
    u : vector::Point3,
    v : vector::Point3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: vector::Point3,
        look_at: vector::Point3,
        vup: vector::Dir3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta * 0.5).tan() * focus_dist;
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let mut w = look_from.clone() - look_at;
        w.normalize();
        let mut u = vector::Vec3::cross(&vup, &w);
        u.normalize();
        let v = vector::Vec3::cross(&w, &u);
        let horizontal = u.clone() * viewport_width;
        let vertical = v.clone() * viewport_height;

        let origin = look_from;
        let lower_left_corner_view =
            -horizontal.clone() / 2.0 - vertical.clone() / 2.0 - w * focus_dist;
        let lower_left_corner_world = origin.clone() + lower_left_corner_view;
        Camera {
            origin: origin,
            lower_left_corner_world: lower_left_corner_world,
            horizontal: horizontal,
            vertical: vertical,
            u : u,
            v : v,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let rd = Camera::random_in_unit_disk() * self.lens_radius;
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        let p = self.lower_left_corner_world.clone()
            + self.horizontal.clone() * s
            + self.vertical.clone() * t;

        let dir = p - &self.origin - &offset;
        ray::Ray::new(self.origin.clone() + offset, dir)
    }
    fn random_in_unit_disk() -> vector::Vec3 {
        loop {
            let p = vector::Vec3::new(
                random::generate_range(-1.0, 1.0),
                random::generate_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}
