pub trait Decodable: std::marker::Send + std::marker::Sync {
    fn set_size(&mut self, w: usize, h: usize);
    fn set_channel(&mut self, c: usize);
    fn set_pixels(&mut self, pixels: Vec<u8>);
}
