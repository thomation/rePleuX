use crate::hit::record::HitRecord;
use crate::math::ray;
use super::scatter;

pub trait Material {
    fn scatter(&self, ray_in: &ray::Ray, hit_record: &HitRecord) -> Option<scatter::ScatterResult>;
}