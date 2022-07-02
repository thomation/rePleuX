use crate::hit::hittable;
use crate::hit::record;
use crate::math::ray;
use crate::math::vector;
use crate::material::material;

pub struct Sphere<M : material::Material> {
    center: vector::Point3,
    radius: f64,
    material : M, 
}

impl<M: material::Material> Sphere<M>{
    pub fn new(center: vector::Point3, radius: f64, material: M) -> Sphere<M> {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
    pub fn center(&self) -> vector::Point3 {
        self.center.clone()
    }
}

impl<M: material::Material> hittable::Hittable for Sphere<M> {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<record::HitRecord> {
        let ray_origin = ray.origin().clone();
        let oc = ray_origin - self.center.clone();
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
        let mut outward_norml = (hit_point.clone() - self.center()) / self.radius;
        let front = vector::Vec3::dot(&outward_norml, &ray.dir()) < 0.0;
        outward_norml.normalize();
        Option::Some(
            record::HitRecord::new(hit_point, outward_norml, t, front, &self.material)
        )
    }
}
