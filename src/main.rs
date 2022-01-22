use std::fs::File;
use image::ColorType;
use image::png::PNGEncoder;
mod vector;

fn main() {
    let image_width = 256;
    let image_height = 256;
    let channel = 3;
    let mut pixels = vec![0; image_width * image_height * channel];
    for h in 0..image_height{
        for w in 0..image_width {
            let color = vector::Vec3::new(w as f64 / (image_width - 1) as f64, (image_height - h - 1) as f64 / (image_height - 1) as f64, 0.25);
            pixels[w * channel + h * image_width * channel] = (color.x() * 255.999) as u8;
            pixels[1 + w * channel + h * image_width * channel] = (color.y() * 255.999) as u8;
            pixels[2 + w * channel + h * image_width * channel] = (color.z() * 255.999) as u8;
        }
    }
    let output = File::create("target/output.png").expect("cannot create output.png");
    let encorder = PNGEncoder::new(output);
    encorder.encode(&pixels, image_width as u32, image_height as u32, ColorType::RGB(8)).expect("encode png fail");

}
