use super::hit;
use crate::math::ray;
use crate::math::vector;

pub struct Sphere {
    center: vector::Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: vector::Point3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
    pub fn center(&self) -> vector::Point3 {
        self.center.clone()
    }
}

impl hit::hittable::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<hit::record::HitRecord> {
        let oc = ray.origin() - self.center.clone();
        let rd = ray.dir();
        let a = rd.length_squared();
        let half_b = vector::Vec3::dot(&oc, &rd);
        let c = vector::Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let mut t = (-half_b - discriminant.sqrt()) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discriminant.sqrt()) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }
        let hit_point = ray.at(t);
        let mut hit_normal = hit_point.clone() - self.center();
        let front = vector::Vec3::dot(&hit_normal, &ray.dir()) < 0.0;
        hit_normal.normalize();
        Option::Some(
            hit::record::HitRecord::new(hit_point, hit_normal, t, front)
        )
    }
}
