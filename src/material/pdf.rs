use crate::math::vector;
pub trait Pdf {
    fn value(&self, dir: &vector::Dir3) -> f64;
    fn generate(&self) -> vector::Dir3;
}