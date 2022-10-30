use crate::math;
use crate::pdf::pdf;
pub struct ScatterResult{
    ray: math::ray::Ray,
    attenuation: math::vector::Color3,
    is_specular: bool,
    pdf: pdf::PdfNode,
}
impl ScatterResult {
    pub fn new(ray: math::ray::Ray, attenuation: math::vector::Color3, is_specular: bool, pdf: pdf::PdfNode) -> Self{
        ScatterResult {
            ray,
            attenuation,
            is_specular,
            pdf,
        }
    }
    pub fn ray(&self) -> &math::ray::Ray {
        &self.ray
    }
    pub fn attenuation(&self) -> &math::vector::Color3 {
        &self.attenuation
    }
    pub fn pdf(&self) -> &pdf::PdfNode {
        &self.pdf
    }
}
