use super::encodable;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Png {
    path: String,
}
impl Png {
    pub fn new(path: String) -> Png {
        Png { path: path }
    }
    pub fn save(&self, img: Arc<Mutex<dyn encodable::Encodable>>) {
        let output = File::create(&self.path).expect("cannot create output.png");
        let encorder = PNGEncoder::new(output);
        let w = img.lock().unwrap().width() as u32;
        let h = img.lock().unwrap().height() as u32;
        encorder
            .encode(img.lock().unwrap().pixels(), w, h, ColorType::RGB(8))
            .expect("encode png fail");
    }
}
