use crate::math::vector;
use crate::material::material;

pub struct HitRecord<'a> {
    p: vector::Point3,
    normal: vector::Dir3,
    t: f64,
    front_face: bool,
    material :&'a material::Material,
}
impl<'a> HitRecord<'a> {
    pub fn new(p: vector::Point3, outward_normal: vector::Dir3, t: f64, front_face: bool, material: &material::Material) -> HitRecord {
        HitRecord {
            p: p,
            normal: if front_face {outward_normal} else {-outward_normal},
            t: t,
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
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn material (&self) -> &material::Material{
        self.material
    }
}
