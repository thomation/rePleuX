pub trait Encodable {
    fn get_size(&self) -> (usize, usize);
    fn pixels(&self) -> &Vec<u8>;
}