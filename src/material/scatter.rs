use crate::math;

pub struct ScatterResult {
    ray: math::ray::Ray,
    attenuation: math::vector::Color3,
    pdf: f64,
}
impl ScatterResult {
    pub fn new(ray: math::ray::Ray, attenuation: math::vector::Color3, pdf: f64) -> ScatterResult {
        ScatterResult {
            ray,
            attenuation,
            pdf,
        }
    }
    pub fn ray(&self) -> &math::ray::Ray {
        &self.ray
    }
    pub fn attenuation(&self) -> &math::vector::Color3 {
        &self.attenuation
    }
    pub fn pdf(&self) -> f64 {
        self.pdf
    }
}
