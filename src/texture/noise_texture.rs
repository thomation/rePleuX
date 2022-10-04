use super::texturable;
use crate::math::{perlin, vector};

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: perlin::Perlin,
}
impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: perlin::Perlin::new(),
        }
    }
}
impl texturable::Texturable for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &vector::Point3) -> vector::Color3 {
        vector::Color3::one() * self.noise.noise(p)
        // let x = if p.x() > 0.0 { 1.0 } else { 0.0 };
        // let y = 0.0;
        // let z = 0.0;
        // vector::Color3::new(x, y, z)
    }
}
