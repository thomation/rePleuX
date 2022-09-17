use super::texturable;
use crate::math::vector;
#[derive(Debug, Clone, Copy)]
pub struct SolidTexture {
    color_value: vector::Color3,
}
impl SolidTexture {
    pub fn new(color: vector::Color3) -> SolidTexture {
        SolidTexture {
            color_value: color,
        }
    }
}
impl texturable::Texturable for SolidTexture {
    fn value (&self, u: f64, v:f64, p: &crate::math::vector::Point3) -> crate::math::vector::Color3 {
        self.color_value
    }
}