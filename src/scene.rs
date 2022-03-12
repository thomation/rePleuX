use super::hit::hittable;
use crate::object::sphere;
use crate::math;
pub struct Scene {
    objects : hittable::HittableList,
}
impl Scene {
    pub fn new() -> Scene {
        let mut objects = hittable::HittableList::new();
        objects.add(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 0.0, -1.0),
            0.5,
        )));
        objects.add(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -100.5, -1.0),
            100.0,
        )));
        Scene {
            objects: objects,
        }
    }
    pub fn objects(&self) -> &hittable::HittableList {
        &self.objects
    }
}
