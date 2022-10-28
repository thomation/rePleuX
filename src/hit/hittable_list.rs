use super::hittable::{self, Hittable};
use crate::math;
use super::record;
use std::sync::Arc;
use super::aabb::AABB;
pub struct HittableList {
    objects: Vec<Arc<dyn hittable::Hittable>>,
}
impl HittableList {
    pub fn new () -> HittableList {
        let objects = vec![];
        HittableList { objects: objects }
    }
    pub fn add (&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}
impl hittable::Hittable for HittableList {
    fn hit(&self, ray: &math::ray::Ray, t_min: f64, t_max: f64) -> std::option::Option<record::HitRecord> {
        let mut hit_anything = std::option::Option::<record::HitRecord>::None;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            let hit = obj.hit(&ray, t_min, closest_so_far);
            match hit {
                Option::Some(r) => {
                    closest_so_far = r.t();
                    hit_anything = Option::Some(r);
                }
                Option::None => {}
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return Option::None;
        }
        let mut final_box = AABB::new(math::vector::Point3::zero(), math::vector::Point3::zero());
        let mut first_box = true;
        for obj in &self.objects {
            let check = obj.bounding_box(time0, time1);
            match check {
                Option::Some(b) => {
                    if first_box {
                        final_box = b;
                    } else {
                        final_box = AABB::surrounding_box(final_box, b)
                    }
                    first_box = false;
                }
                Option::None => {
                    return Option::None;
                }
            }
        }
        Option::Some(final_box)
    }

    fn pdf_value(&self, o: &math::vector::Point3, v: &math::vector::Dir3) -> f64 {
        todo!()
    }

    fn random(&self, o: &math::vector::Point3) -> math::vector::Dir3 {
        todo!()
    }
}
