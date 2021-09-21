use std::fs::File;
use image::ColorType;
use image::png::PNGEncoder;

fn main() {
    let image_width = 256;
    let image_height = 256;
    let channel = 3;
    let mut pixels = vec![0; image_width * image_height * channel];
    for h in 0..image_height {
        for w in 0..image_width {
            let r = w as f32 / (image_width - 1) as f32;
            let g = h as f32 / (image_height - 1) as f32;
            let b = 0.25;
            // pixels[w + h * image_width + 0] = ;
            // pixels[w + h * image_width + 1] = ;
            // pixels[w + h * image_width + 2] = ;
            pixels[w * channel + h * image_width * channel] = (r * 255.999) as u8;
            pixels[1 + w * channel + h * image_width * channel] = (g * 255.999) as u8;
            pixels[2 + w * channel + h * image_width * channel] = (b * 255.999) as u8;
        }
    }
    let output = File::create("target/output.png").expect("cannot create output.png");
    let encorder = PNGEncoder::new(output);
    encorder.encode(&pixels, image_width as u32, image_height as u32, ColorType::RGB(8)).expect("encode png fail");

}
