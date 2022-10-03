use super::decodable;
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
        let size = img.lock().unwrap().get_size();
        encorder
            .encode(img.lock().unwrap().pixels(), size.0 as u32, size.1 as u32, ColorType::RGB(8))
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
    pub fn load(&mut self, img: &mut dyn decodable::Decodable) {
        match self.decorder.read_image() {
            Ok(d) => match d {
                image::DecodingResult::U8(u) => img.set_pixels(u),
                image::DecodingResult::U16(_) => panic!("Not support u16"),
            },
            Err(_) => panic!("Cannot decode jpeg file"),
        }
        match self.decorder.dimensions() {
            Ok(d) => {
                img.set_size(d.0 as usize, d.1 as usize);
            }
            Err(_) => panic!("Cannot get dimensions"),
        }
        match self.decorder.colortype() {
            Ok(t) => match t {
                ColorType::Gray(_) => panic!("Does not support Gray"),
                ColorType::RGB(_) => img.set_channel(3),
                ColorType::Palette(_) => panic!("Does not support Palette"),
                ColorType::GrayA(_) => panic!("Does not support GrayA"),
                ColorType::RGBA(_) => panic!("Does not support RGBA"),
            },
            Err(_) => panic!("Cannot get color type"),
        }
    }
}
