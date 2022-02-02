use crate::vector;
struct Ray {
    origin: vector::Point3,
    dir: vector::Vec3,
}
impl Ray {
    fn new(origin: vector::Point3, dir: vector::Vec3) -> Ray {
        Ray {
            origin: origin,
            dir: dir,
        }
    }
    fn origin(&self) -> &vector::Point3 {
        &self.origin
    }
    fn dir(&self) -> &vector::Vec3 {
        &self.dir
    }
    fn at(&self, t: f64) -> vector::Point3 {
        // self.origin() + self.dir() * t
        vector::Point3::new(self.origin().x() + self.dir().x() * t, self.origin().y() + self.dir().y() * t, self.origin().z() + self.dir().z() * t)
    }
}
#[test]
fn test_ray() {
    let r = Ray::new(
        vector::Point3::new(0.0, 0.0, 0.0),
        vector::Vec3::new(1.0, 2.0, 3.0),
    );
    assert_eq!(*r.origin(), vector::Point3::new(0.0, 0.0, 0.0));
    assert_eq!(*r.dir(), vector::Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(r.at(0.5), vector::Point3::new(0.5, 1.0, 1.5));
    assert_eq!(r.at(-0.5), vector::Point3::new(-0.5, -1.0, -1.5));
}
