
use crate::math::vector::Color3;
pub trait Decodable: std::marker::Send + std::marker::Sync {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn color(&self, w: usize, h: usize) -> Color3;
}