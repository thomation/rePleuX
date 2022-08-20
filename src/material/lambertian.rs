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
    pub fn albedo(&self) -> &vector::Color3 {
        &self.albedo
    }
}
impl material::Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &HitRecord,
    ) -> Option<scatter::ScatterResult> {
        let mut scatter_dir = hit_record.normal().clone() + material::random_in_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal().clone();
        }
        let ray_out = math::ray::Ray::new(hit_record.position().clone(), scatter_dir, ray_in.time());
        Option::Some(scatter::ScatterResult::new(ray_out, self.albedo().clone()))
    }
}
