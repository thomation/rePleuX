use crate::math::vector;
pub trait Texturable {
    fn color (u: f64, v:f64, p: &vector::Point3) -> vector::Color3;
}