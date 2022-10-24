use std::sync::Arc;
use crate::math::vector;
use crate::material::material;

#[derive(Clone)]
pub struct HitRecord {
    p: vector::Point3,
    normal: vector::Dir3,
    t: f64,
    u: f64,
    v: f64,
    front_face: bool,
    material :Arc<dyn material::Material>,
}
impl HitRecord {
    pub fn new(p: vector::Point3, outward_normal: vector::Dir3, t: f64, u: f64, v:f64, front_face: bool, material: Arc<dyn material::Material>) -> HitRecord {
        HitRecord {
            p: p,
            normal: if front_face {outward_normal} else {-outward_normal},
            t: t,
            u: u,
            v: v,
            front_face: front_face,
            material : material, 
        }
    }
    pub fn position(&self) -> &vector::Point3{
        &self.p
    }
    pub fn normal(&self) -> &vector::Dir3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn u(&self) -> f64 {
        self.u
    }
    pub fn v(&self) -> f64 {
        self.v
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn set_front_face(&mut self, front_face: bool) {
        self.front_face = front_face
    }
    pub fn material (&self) -> Arc<dyn material::Material>{
        self.material.clone()
    }
}
