use rand::Rng;
pub fn generate() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
pub fn generate_range(min: f64, max: f64) -> f64 {
    let r = generate();
    min + r * (max - min)
}
