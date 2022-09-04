use rand::Rng;
pub fn generate() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
pub fn generate_range(min: f64, max: f64) -> f64 {
    let r = generate();
    min + r * (max - min)
}
pub fn generate_range_int(min: usize, max: usize) -> usize {
    let r = generate_range(min as f64, (max + 1) as f64);
    r as usize
}

#[test]
fn test_random() {
    let mut has_min = false;
    let mut has_max = false;
    for _ in 0..1000 {
        let r = generate_range_int(1, 3);
        assert!(r == 1 || r == 2 || r == 3, "unvalid value:{}", r);
        if r == 1 {
            has_min = true;
        }
        if r == 3 {
            has_max = true;
        }
    }
    assert!(has_max, "without max");
    assert!(has_min, "without min");
}
