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
    decorder: JPEGDecoder<File>,
}
impl Jpeg {
    pub fn new(path: String) -> Jpeg {
        let input = File::open(path).expect("cannot read intput file");
        let decorder = JPEGDecoder::new(input);
        Jpeg { decorder }
    }
    pub fn load(&mut self) -> Vec<u8> {
        match self.decorder.read_image() {
            Ok(d) => match d {
                image::DecodingResult::U8(u) => u,
                image::DecodingResult::U16(_) => panic!("Not support u16"),
            },
            Err(_) => panic!("Cannot decode jpeg file"),
        }
    }
    pub fn Size(&mut self) ->(usize, usize) {
        match self.decorder.dimensions() {
            Ok(d) => (d.0 as usize, d.1 as usize),
            Err(_) => panic!("Cannot get dimensions"),
        }
    }
    pub fn Channel(&mut self) -> usize {
        match  self.decorder.colortype() {
            Ok(t) => {match t {
                ColorType::Gray(_) => panic!("Does not support Gray"),
                ColorType::RGB(_) => 3,
                ColorType::Palette(_) => panic!("Does not support Palette"),
                ColorType::GrayA(_) => panic!("Does not support GrayA"),
                ColorType::RGBA(_) => panic!("Does not support RGBA"), 
            }},
            Err(_) => panic!("Cannot get color type"),
        }
    }
}
