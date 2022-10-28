use super::scatter;
use crate::hit::record::HitRecord;
use crate::math::{random, ray, vector};

pub trait Material: std::marker::Send + std::marker::Sync {
    fn scatter(&self, ray_in: &ray::Ray, hit_record: &HitRecord) -> Option<scatter::ScatterResult>;
    fn emitted(&self, hit_record: &HitRecord, u: f64, v: f64, p: &vector::Point3) -> vector::Color3;
    fn scatting_pdf(&self, hit_record: &HitRecord, scattered: &ray::Ray) -> f64;
}
