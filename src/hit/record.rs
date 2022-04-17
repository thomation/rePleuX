use crate::math::vector;
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
    pub fn position(&self) -> vector::Point3{
        self.p.clone()
    }
    pub fn normal(&self) -> vector::Dir3 {
        self.normal.clone()
    }
    pub fn t(&self) -> f64 {
        self.t
    }
}
