use super::rect;
use crate::hit::hittable_list;
use crate::material::material;
use crate::math::vector;
use std::sync::Arc;
pub struct Cubic {
    box_min: vector::Point3,
    box_max: vector::Point3,
    sides: hittable_list::HittableList,
}
impl Cubic {
    pub fn new(
        p0: vector::Point3,
        p1: vector::Point3,
        material: Arc<dyn material::Material>,
    ) -> Cubic {
        let mut sides = hittable_list::HittableList::new();
        sides.add(Arc::new(rect::XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        )));
        sides.add(Arc::new(rect::XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        )));
        sides.add(Arc::new(rect::XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        )));
        sides.add(Arc::new(rect::XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        )));
        sides.add(Arc::new(rect::YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material.clone(),
        )));
        sides.add(Arc::new(rect::YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material.clone(),
        )));
        Cubic {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
