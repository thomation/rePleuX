use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use super::encodable;

pub struct ImageFile {
    path: String,
}
impl ImageFile {
    pub fn new(path: String) -> ImageFile {
        ImageFile { path: path }
    }
    pub fn save(&self, img: &encodable::Encodable) {
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
