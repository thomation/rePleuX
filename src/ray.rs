use crate::vector;
pub struct Ray {
    origin: vector::Point3,
    dir: vector::Vec3,
}
impl Ray {
    pub fn new(origin: vector::Point3, dir: vector::Vec3) -> Ray {
        Ray {
            origin: origin,
            dir: dir,
        }
    }
    pub fn origin(&self) -> vector::Point3 {
        self.origin.clone()
    }
    pub fn dir(&self) -> vector::Vec3 {
        self.dir.clone()
    }
    pub fn at(&self, t: f64) -> vector::Point3 {
        self.origin() + self.dir() * t
    }
}
#[test]
fn test_ray() {
    let r = Ray::new(
        vector::Point3::new(0.0, 0.0, 0.0),
        vector::Vec3::new(1.0, 2.0, 3.0),
    );
    assert_eq!(r.origin(), vector::Point3::new(0.0, 0.0, 0.0));
    assert_eq!(r.dir(), vector::Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(r.at(0.5), vector::Point3::new(0.5, 1.0, 1.5));
    assert_eq!(r.at(-0.5), vector::Point3::new(-0.5, -1.0, -1.5));
}
