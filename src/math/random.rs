use rand::Rng;
use super::vector;

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

pub fn random_in_unit_sphere() -> vector::Vec3 {
    loop {
        let p = vector::Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
pub fn random_in_unit_vector() -> vector::Vec3 {
    let mut r = random_in_unit_sphere();
    r.normalize();
    r
}
pub fn random_in_half_sphere(normal: &vector::Dir3) -> vector::Vec3 {
    let unit_sphere = random_in_unit_sphere();
    if vector::Vec3::dot(&unit_sphere, normal) < 0.0 {
        return -unit_sphere;
    }
    unit_sphere
}
pub fn random_cosine_direction() -> vector::Dir3 {
    let r1 = generate();
    let r2 = generate();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * std::f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    vector::Dir3::new(x, y, z)
}
pub fn random_to_sphere(radius: f64, distance_squared: f64) -> vector::Dir3 {
    let r1 = generate();
    let r2 = generate();
    let z = 1.0 + r2*((1.0-radius*radius/distance_squared).sqrt() - 1.0);

    let phi = 2.0*std::f64::consts::PI*r1;
    let x = phi.cos()*(1.0-z*z).sqrt();
    let y = phi.cos()*(1.0-z*z).sqrt();

    vector::Dir3::new(x, y, z)
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
