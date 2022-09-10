use crate::math::vector;
pub trait Texturable {
    fn value (&self, u: f64, v:f64, p: &vector::Point3) -> vector::Color3;
}