use crate::hit::aabb;
use crate::hit::hittable;
use std::sync::Arc;
pub struct BvhNode {
    bounding_box: aabb::AABB,
    left: Arc<dyn hittable::Hittable>,
    right: Arc<dyn hittable::Hittable>,
}
impl BvhNode {
    pub fn new(objects: &Vec<Arc<dyn hittable::Hittable>>, time0: f64, time1: f64) -> BvhNode {
        BvhNode::do_new(objects, 0, objects.len(), time0, time1)
    }
    fn do_new(
        objects: &Vec<Arc<dyn hittable::Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let object_span = end - start;
        let mut left;
        let mut right;
        if object_span == 1 {
            left = match objects.get(start) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index"),
            };
            right = match objects.get(start) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index"),
            };
        } else if object_span == 2 {
            // TODO: compare
            left = match objects.get(start) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index"),
            };
            right = match objects.get(start + 1) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index"),
            };
        } else {
            // TODO: sort
            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::do_new(objects, start, mid, time0, time1));
            right = Arc::new(BvhNode::do_new(objects, mid, end, time0, time1));
        }
        let left_box = match left.bounding_box(time0, time1) {
            Option::Some(b) => b,
            Option::None => {
                panic!("No bounding")
            }
        };
        let right_box = match right.bounding_box(time0, time1) {
            Option::Some(b) => b,
            Option::None => {
                panic!("No bounding")
            }
        };
        BvhNode {
            bounding_box: aabb::AABB::surrounding_box(left_box, right_box),
            left: left.clone(),
            right: right.clone(),
        }
    }
}
impl hittable::Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hit::record::HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return Option::None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        match hit_left {
            Option::Some(r) => Option::Some(r),
            Option::None => {
                let hit_right = self.right.hit(ray, t_min, t_max);
                match hit_right {
                    Option::Some(r) => Option::Some(r),
                    Option::None => Option::None,
                }
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<aabb::AABB> {
        Option::Some(self.bounding_box)
    }
}
