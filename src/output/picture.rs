use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use super::encodable;

pub struct Png {
    path: String,
}
impl Png {
    pub fn new(path: String) -> Png {
        Png { path: path }
    }
    pub fn save(&self, img: &impl encodable::Encodable) {
        let output = File::create(&self.path).expect("cannot create output.png");
        let encorder = PNGEncoder::new(output);
        encorder
            .encode(
                img.pixels(),
                img.width() as u32,
                img.height() as u32,
                ColorType::RGB(8),
            )
            .expect("encode png fail");
    }
}
