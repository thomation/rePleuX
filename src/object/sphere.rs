use crate::hit::hittable;
use crate::hit::record;
use crate::material::material;
use crate::math::ray;
use crate::math::vector;

pub struct Sphere<M: material::Material> {
    center0: vector::Point3,
    center1: vector::Point3,
    radius: f64,
    material: M,
    time0: f64,
    time1: f64,
}

impl<M: material::Material> Sphere<M> {
    pub fn new(center: vector::Point3, radius: f64, material: M) -> Sphere<M> {
        Sphere {
            center0: center,
            center1: center,
            radius: radius,
            material: material,
            time0: 0.0,
            time1: 0.0,
        }
    }
    pub fn new_move(center0: vector::Point3, center1: vector::Point3, radius: f64, material:M, time0:f64, time1:f64) -> Sphere<M> {
        Sphere {
            center0: center0,
            center1: center1,
            radius: radius,
            material: material,
            time0: time0,
            time1: time1,
        }
    }
    pub fn center(&self, time: f64) -> vector::Point3 {
        if self.time0 == self.time1 {
            return self.center0;
        }
        let v = (time - self.time0) / (self.time1 - self.time0);
        self.center0 + (self.center1 - self.center0) * v
    }
}

impl<M: material::Material + std::marker::Send + std::marker::Sync> hittable::Hittable
    for Sphere<M>
{
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> std::option::Option<record::HitRecord> {
        let ray_origin = ray.origin().clone();
        let oc = ray_origin - &self.center(ray.time());
        let ray_dir = ray.dir();
        let a = ray_dir.length_squared();
        let half_b = vector::Vec3::dot(&oc, ray_dir);
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
        let mut outward_norml = (hit_point.clone() - self.center(ray.time())) / self.radius;
        let front = vector::Vec3::dot(&outward_norml, &ray.dir()) < 0.0;
        outward_norml.normalize();
        Option::Some(record::HitRecord::new(
            hit_point,
            outward_norml,
            t,
            front,
            &self.material,
        ))
    }
}
