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
        let ps = p.clone() * self.scale;
        vector::Color3::one() * 0.5 * (1.0 + self.noise.noise(&ps))
    }
}
