use super::hittable_list;
use super::sphere;
use crate::math;
pub struct World {
    objects : hittable_list::HittableList,
}
impl World {
    pub fn new() -> World {
        let mut objects = hittable_list::HittableList::new();
        objects.add(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, 0.0, -1.0),
            0.5,
        )));
        objects.add(Box::new(sphere::Sphere::new(
            math::vector::Point3::new(0.0, -100.5, -1.0),
            100.0,
        )));
        World {
            objects: objects,
        }
    }
    pub fn objects(&self) -> &hittable_list::HittableList {
        &self.objects
    }
}
