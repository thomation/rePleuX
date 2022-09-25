use crate::math::vector;
pub trait Texturable: std::marker::Send + std::marker::Sync {
    fn value(&self, u: f64, v: f64, p: &vector::Point3) -> vector::Color3;
}
