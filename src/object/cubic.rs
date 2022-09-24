use super::rect;
use crate::hit::hittable_list;
use crate::material::material;
use crate::math::vector;
pub struct Cubic {
    box_min: vector::Point3,
    box_max: vector::Point3,
    sides: hittable_list::HittableList,
}
impl Cubic {
    pub fn new<M: material::Material>(
        p0: vector::Point3,
        p1: vector::Point3,
        material: M,
    ) -> Cubic {
        let sides = hittable_list::HittableList::new();
        // sides.add(Arc::new(rect::XYRect::new(
        //     p0.x(),
        //     p1.x(),
        //     p0.y(),
        //     p1.y(),
        //     p1.z(),
        //     material,
        // )));
        // sides.add(Arc::new(rect::XYRect::new(
        //     p0.x(),
        //     p1.x(),
        //     p0.y(),
        //     p1.y(),
        //     p0.z(),
        //     material,
        // )));
        // sides.add(Arc::new(rect::XZRect::new(
        //     p0.x(),
        //     p1.x(),
        //     p0.z(),
        //     p1.z(),
        //     p1.y(),
        //     material,
        // )));
        // sides.add(Arc::new(rect::XZRect::new(
        //     p0.x(),
        //     p1.x(),
        //     p0.z(),
        //     p1.z(),
        //     p0.y(),
        //     material,
        // )));
        // sides.add(Arc::new(rect::YZRect::new(
        //     p0.y(),
        //     p1.y(),
        //     p0.z(),
        //     p1.z(),
        //     p1.x(),
        //     material,
        // )));
        // sides.add(Arc::new(rect::YZRect::new(
        //     p0.y(),
        //     p1.y(),
        //     p0.z(),
        //     p1.z(),
        //     p0.x(),
        //     material,
        // )));
        Cubic {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
