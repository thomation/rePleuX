use crate::hit::{hittable, record};
use std::sync::Arc;

pub struct FlipFace {
    hittable: Arc<dyn hittable::Hittable>,
}
impl FlipFace {
    pub fn new(hittable: Arc<dyn hittable::Hittable>) -> FlipFace {
        FlipFace { hittable }
    }
}
impl hittable::Hittable for FlipFace {
    fn hit(&self, ray: &crate::math::ray::Ray, t_min: f64, t_max: f64) -> Option<record::HitRecord> {
        match self.hittable.hit(ray, t_min, t_max) {
            Some(r) => {
                let mut rr = r.clone();
                rr.set_front_face(!rr.front_face());
                Option::Some(rr) 
            },
            None => Option::None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::hit::aabb::AABB> {
        self.hittable.bounding_box(time0, time1)
    }
}
