use super::encodable;
use image::jpeg::JPEGDecoder;
use image::png::PNGEncoder;
use image::ColorType;
use image::ImageDecoder;
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
pub struct Jpeg {
    path: String,
}
impl Jpeg {
    pub fn new(path: String) -> Jpeg {
        Jpeg { path }
    }
    pub fn load(&self) -> Vec<u8> {
        let input = File::open(&self.path).expect("cannot read intput file");
        let mut decorder = JPEGDecoder::new(input);
        match decorder.read_image() {
            Ok(d) => match d {
                image::DecodingResult::U8(u) => u,
                image::DecodingResult::U16(_) => panic!("Not support u16"),
            },
            Err(_) => panic!("Cannot decode jpeg file"),
        }
    }
}
