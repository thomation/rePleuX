use super::scatter;
use crate::hit::record::HitRecord;
use crate::math::ray;
use crate::math::vector;

pub trait Material {
    fn scatter(&self, ray_in: &ray::Ray, hit_record: &HitRecord) -> Option<scatter::ScatterResult>;
    fn emitted(&self, u: f64, v: f64, p: &vector::Point3) -> vector::Color3;
}
pub fn random_in_unit_sphere() -> vector::Vec3 {
    loop {
        let p = vector::Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
pub fn random_in_unit_vector() -> vector::Vec3 {
    let mut r = random_in_unit_sphere();
    r.normalize();
    r
}
pub fn random_in_half_sphere(normal: &vector::Dir3) -> vector::Vec3 {
    let unit_sphere = random_in_unit_sphere();
    if vector::Vec3::dot(&unit_sphere, normal) < 0.0 {
        return -unit_sphere;
    }
    unit_sphere
}
