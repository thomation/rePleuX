use crate::ray;
use crate::vector;
pub struct HitRecord {
    p: vector::Point3,
    normal: vector::Dir3,
    t: f64,
    front_face: bool,
}
impl HitRecord {
    pub fn new(p: vector::Point3, normal: vector::Dir3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }
    pub fn normal(&self) -> &vector::Dir3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
}
pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
