use crate::hit::record::HitRecord;
use crate::math::ray;
use crate::math::vector;

pub trait Material {
    fn scatter(&self, ray_in: &ray::Ray, hit_record: &HitRecord) -> Option<ray::Ray>;
}