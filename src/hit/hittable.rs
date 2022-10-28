use super::aabb::AABB;
use super::record::HitRecord;
use crate::math::{ray, vector};

pub trait Hittable: std::marker::Send + std::marker::Sync {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64;
    fn random(&self, o: &vector::Point3) -> vector::Dir3;
}
