use crate::math;

pub struct ScatterResult {
    ray: math::ray::Ray,
    attenuation: math::vector::Color3,
}
impl ScatterResult {
    pub fn new(ray: math::ray::Ray, attenuation: math::vector::Color3) -> ScatterResult {
        ScatterResult {
            ray: ray,
            attenuation: attenuation,
        }
    }
    pub fn ray(&self) -> &math::ray::Ray {
        &self.ray
    }
    pub fn attenuation(&self) -> math::vector::Color3 {
        self.attenuation.clone()
    }
}
