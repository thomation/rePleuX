use crate::hit::{self, aabb, hittable, record};
use crate::math::{ray, vector};
use std::sync::Arc;
pub struct Translate {
    hittable: Arc<dyn hittable::Hittable>,
    offset: vector::Dir3,
}
impl Translate {
    pub fn new(hittable: Arc<dyn hittable::Hittable>, offset: vector::Dir3) -> Translate {
        Translate { hittable, offset }
    }
}

impl hittable::Hittable for Translate {
    fn hit(
        &self,
        ray: &crate::math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<record::HitRecord> {
        let moved_ray = ray::Ray::new(
            ray.origin().clone() - self.offset,
            ray.dir().clone(),
            ray.time(),
        );
        let check = self.hittable.hit(&moved_ray, t_min, t_max);
        match check {
            Some(r) =>{
                let p = r.position().clone() + self.offset;
                let front_face = vector::Vec3::dot(r.normal(), moved_ray.dir()) < 0.0;
                Option::Some(record::HitRecord::new(p,r.normal().clone() , r.t(), r.u(), r.v(), front_face, r.material()))
            },
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<hit::aabb::AABB> {
        let check = self.hittable.bounding_box(time0, time1);
        match check {
            Some(b) => Option::Some(aabb::AABB::new(
                b.min().clone() + self.offset,
                b.max().clone() + self.offset,
            )),
            None => None,
        }
    }
}
