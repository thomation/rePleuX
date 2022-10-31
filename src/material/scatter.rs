use crate::math;
use crate::pdf::pdf;
pub enum SpecularValue {
    Value(math::ray::Ray),
    Null,
}
pub struct ScatterResult{
    specular: SpecularValue,
    attenuation: math::vector::Color3,
    pdf: pdf::PdfNode,
}
impl ScatterResult {
    pub fn new(attenuation: math::vector::Color3, pdf: pdf::PdfNode, specular: SpecularValue) -> Self{
        ScatterResult {
            specular,
            attenuation,
            pdf,
        }
    }
    pub fn attenuation(&self) -> &math::vector::Color3 {
        &self.attenuation
    }
    pub fn pdf(&self) -> &pdf::PdfNode {
        &self.pdf
    }
}
