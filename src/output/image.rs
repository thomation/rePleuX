use crate::vector;
use super::encodable;

pub struct Image {
    image_width: usize,
    image_height: usize,
    pixels: Vec<u8>,
    channel: usize,
}

impl Image {
    pub fn new(image_width: usize, image_height: usize) -> Image {
        let channel = 3;
        let pixels = vec![0; image_width * image_height * channel];
        Image {
            image_width: image_width,
            image_height: image_height,
            pixels: pixels,
            channel: channel,
        }
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
impl encodable::Encodable for Image {
    fn width(&self) -> usize {
        self.image_width
    }
    fn height(&self) -> usize {
        self.image_height
    }
    fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }
}