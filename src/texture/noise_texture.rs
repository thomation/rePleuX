use super::texturable;
use crate::math::{perlin, vector};

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: perlin::Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: perlin::Perlin::new(),
            scale,
        }
    }
}
impl texturable::Texturable for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &vector::Point3) -> vector::Color3 {
        vector::Color3::one() * 0.5 * (1.0 + (p.z()*self.scale + 10.0 * self.noise.turb(&p, 7)).sin())
    }
}
