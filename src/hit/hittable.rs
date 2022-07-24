use super::record::HitRecord;
use crate::math::ray;

pub trait Hittable : std::marker::Send + std::marker::Sync{
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

