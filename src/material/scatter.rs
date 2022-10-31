use crate::math;
use crate::pdf::pdf;
pub enum SpecularValue {
    Value(math::ray::Ray),
    Null,
}
pub struct ScatterResult{
    specular: SpecularValue,
    attenuation: math::vector::Color3,
    pdf: pdf::PdfValue,
}
impl ScatterResult {
    pub fn new(attenuation: math::vector::Color3, pdf: pdf::PdfValue, specular: SpecularValue) -> Self{
        ScatterResult {
            specular,
            attenuation,
            pdf,
        }
    }
    pub fn attenuation(&self) -> &math::vector::Color3 {
        &self.attenuation
    }
    pub fn pdf(&self) -> &pdf::PdfValue {
        &self.pdf
    }
    pub fn specular(&self) -> &SpecularValue {
        &self.specular
    }
}
