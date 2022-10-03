pub trait Encodable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &Vec<u8>;
}