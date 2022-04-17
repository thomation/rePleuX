use rand::Rng;
pub fn generate() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
