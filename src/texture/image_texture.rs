use super::texturable;
use crate::io::decodable;
use std::sync::Arc;
pub struct ImageTexture {
    data: Arc<dyn decodable::Decodable>,
}
impl ImageTexture {
    pub fn new(data: Arc<dyn decodable::Decodable>) -> ImageTexture {
        ImageTexture { data }
    }
}

impl texturable::Texturable for ImageTexture {
    fn value(
        &self,
        raw_u: f64,
        raw_v: f64,
        p: &crate::math::vector::Point3,
    ) -> crate::math::vector::Color3 {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = raw_u.clamp(0.0, 1.0);
        let v = 1.0 - raw_v.clamp(0.0, 1.0);  // Flip V to image coordinates
        let width = self.data.width();
        let height = self.data.height();
        let mut i = (u * width as f64) as usize;
        let mut j = (v * height as f64) as usize;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= width {
            i = width - 1;
        }
        if j >= height {
            j = height - 1;
        }

        self.data.color(i, j)
    }
}
