use crate::vector;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

pub struct Image {
    image_width: usize,
    image_height: usize,
    pixels: Vec<u8>,
    channel: usize,
}

impl Image {
    pub fn new(aspect_ratio: f64) -> Image {
        let image_width = 800;
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let channel = 3;
        let pixels = vec![0; image_width * image_height * channel];
        Image {
            image_width: image_width,
            image_height: image_height,
            pixels: pixels,
            channel: channel,
        }
    }
    pub fn width(&self) -> usize {
        self.image_width
    }
    pub fn height(&self) -> usize {
        self.image_height
    }
    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }
    pub fn write_color(&mut self, w: usize, h: usize, color: &vector::Color3) {
        self.pixels[w * self.channel + h * self.image_width * self.channel] =
            (color.x() * 255.999) as u8;
        self.pixels[1 + w * self.channel + h * self.image_width * self.channel] =
            (color.y() * 255.999) as u8;
        self.pixels[2 + w * self.channel + h * self.image_width * self.channel] =
            (color.z() * 255.999) as u8;
    }
}

pub struct ImageFile {
    path: String,
}
impl ImageFile {
    pub fn new(path: String) -> ImageFile {
        ImageFile {
            path: path,
        }
    }
    pub fn save(&self, img: &Image) {
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
