use super::material;
use super::scatter;
use crate::hit::record::HitRecord;
use crate::math;
use crate::math::vector;

pub struct Lambertian {
    albedo: vector::Color3,
}
impl Lambertian {
    pub fn new(albedo: vector::Color3) -> Lambertian {
        Lambertian { albedo: albedo }
    }
    pub fn albedo(&self) -> vector::Color3 {
        self.albedo.clone()
    }
    fn random_in_unit_sphere() -> math::vector::Vec3 {
        loop {
            let p = math::vector::Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    fn random_in_unit_vector() -> math::vector::Vec3 {
        let mut r = Lambertian::random_in_unit_sphere();
        r.normalize();
        r
    }
    fn random_in_half_sphere(normal: &math::vector::Dir3) -> math::vector::Vec3 {
        let unit_sphere = Lambertian::random_in_unit_sphere();
        if math::vector::Vec3::dot(&unit_sphere, normal) < 0.0 {
            return -unit_sphere;
        }
        unit_sphere
    }
}
impl material::Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &HitRecord,
    ) -> Option<scatter::ScatterResult> {
        let mut scatter_dir = hit_record.normal() + Lambertian::random_in_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal();
        }
        let ray_out = math::ray::Ray::new(hit_record.position(), scatter_dir);
        Option::Some(scatter::ScatterResult::new(ray_out, self.albedo()))
    }
}
