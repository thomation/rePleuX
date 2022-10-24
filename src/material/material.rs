use super::scatter;
use crate::hit::record::HitRecord;
use crate::math::{random, ray, vector};

pub trait Material: std::marker::Send + std::marker::Sync {
    fn scatter(&self, ray_in: &ray::Ray, hit_record: &HitRecord) -> Option<scatter::ScatterResult>;
    fn emitted(&self, hit_record: &HitRecord, u: f64, v: f64, p: &vector::Point3) -> vector::Color3;
    fn scatting_pdf(&self, hit_record: &HitRecord, scattered: &ray::Ray) -> f64;
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
pub fn random_cosine_direction() -> vector::Dir3 {
    let r1 = random::generate();
    let r2 = random::generate();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * std::f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    vector::Dir3::new(x, y, z)
}
