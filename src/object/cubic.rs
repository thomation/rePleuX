use super::rect;
use crate::hit::{hittable_list, hittable, aabb};
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
impl hittable::Hittable for Cubic {
    fn hit(&self, ray: &crate::math::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hit::record::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::hit::aabb::AABB> {
        Option::Some(aabb::AABB::new(self.box_min, self.box_max))
    }

    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64 {
        todo!()
    }

    fn random(&self, o: &vector::Point3) -> vector::Dir3 {
        todo!()
    }
}