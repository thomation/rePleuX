use crate::math::vector;
use super::encodable;

pub struct Bitmap {
    image_width: usize,
    image_height: usize,
    pixels: Vec<u8>,
    channel: usize,
    finished_count: i32,
    max_count: usize,
}

impl Bitmap {
    pub fn new(image_width: usize, image_height: usize) -> Bitmap {
        let channel = 3;
        let pixels = vec![0; image_width * image_height * channel];
        Bitmap {
            image_width: image_width,
            image_height: image_height,
            pixels: pixels,
            channel: channel,
            finished_count : 0,
            max_count: image_height * image_width,
        }
    }
    pub fn write_color(&mut self, w: usize, h: usize, raw_color: &vector::Color3) {
        let color = vector::Color3::new(raw_color.x().sqrt(), raw_color.y().sqrt(), raw_color.z().sqrt());
        self.pixels[w * self.channel + h * self.image_width * self.channel] =
            (color.x() * 255.999) as u8;
        self.pixels[1 + w * self.channel + h * self.image_width * self.channel] =
            (color.y() * 255.999) as u8;
        self.pixels[2 + w * self.channel + h * self.image_width * self.channel] =
            (color.z() * 255.999) as u8;
        self.finished_count += 1;
        println!("Progress:{}/{}, {}%", self.finished_count, self.max_count, self.finished_count as f32 * 100.0 / self.max_count as f32);
    }
}
impl encodable::Encodable for Bitmap {
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