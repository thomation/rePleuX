use crate::hit::aabb;
use crate::hit::hittable;
use std::sync::Arc;
use crate::math::random;
use crate::hit::record;
pub struct BvhNode {
    bounding_box: aabb::AABB,
    left: Arc<dyn hittable::Hittable>,
    right: Arc<dyn hittable::Hittable>,
}
impl BvhNode {
    pub fn new(objects: &mut Vec<Arc<dyn hittable::Hittable>>, time0: f64, time1: f64) -> BvhNode {
        BvhNode::do_new(objects, time0, time1)
    }
    fn do_new(objects: &mut [Arc<dyn hittable::Hittable>], time0: f64, time1: f64) -> BvhNode {
        let object_span = objects.len();
        let start = 0;
        let end = object_span;
        let mut left;
        let mut right;
        let axis = random::generate_range_int(0, 2);
        if object_span == 1 {
            left = match objects.get(start) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index {} of {} ", start, objects.len()),
            };
            right = left.clone();
        } else if object_span == 2 {
            left = match objects.get(start) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index {} of {} ", start, objects.len()),
            };
            right = match objects.get(start + 1) {
                Option::Some(h) => h.clone(),
                Option::None => panic!("Out of index {} of {}", start + 1, objects.len()),
            };
            let c = BvhNode::compare(&left, &right, axis);
            if c.is_gt() {
                std::mem::swap(&mut left, &mut right);
            }
        } else {
            objects.sort_by(|a, b| BvhNode::compare(a, b, axis));
            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::do_new(&mut objects[start..mid], time0, time1));
            right = Arc::new(BvhNode::do_new(&mut objects[mid..end], time0, time1));
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
    fn compare(
        left: &Arc<dyn hittable::Hittable>,
        right: &Arc<dyn hittable::Hittable>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let lb = match left.bounding_box(0.0, 0.0) {
            Option::Some(b) => b,
            Option::None => panic!("Cannot find bounding box"),
        };
        let rb = match right.bounding_box(0.0, 0.0) {
            Option::Some(b) => b,
            Option::None => panic!("Cannot find bounding box"),
        };
        let lv = lb.min()[axis];
        let rv = rb.min()[axis];
        if lv < rv {
            return std::cmp::Ordering::Less;
        } else if lv > rv {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    }
}
impl hittable::Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<record::HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return Option::None;
        }
        let hits = vec![self.left.hit(ray, t_min, t_max), self.right.hit(ray, t_min, t_max)];
        let mut hit_anything = std::option::Option::<record::HitRecord>::None;
        let mut closest_so_far = t_max;
        for hit in hits {
            match hit {
                Option::Some(r) => {
                    if r.t() < closest_so_far {
                        closest_so_far = r.t();
                        hit_anything = Option::Some(r);
                    }
                }
                Option::None => {}
            }
        }
        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<aabb::AABB> {
        Option::Some(self.bounding_box)
    }
}

use crate::material;
use crate::math;
use crate::object::sphere;
#[test]
fn test_bvh() {
    let mut objects: Vec<Arc<dyn hittable::Hittable>> = vec![];
    for a in -2..2 {
        for b in -2..2 {
            for c in -2..2 {
                let center = math::vector::Point3::new(
                    a as f64 + 10.0 * math::random::generate(),
                    b as f64 + 10.0 * math::random::generate(),
                    c as f64 + 10.0 * math::random::generate(),
                );
                let albedo = math::vector::Color3::random() * math::vector::Color3::random();
                let center2 = center
                    + math::vector::Vec3::new(0.0, math::random::generate_range(0.0, 0.5), 0.0);
                objects.push(Arc::new(sphere::Sphere::new_move(
                    center,
                    center2,
                    0.2,
                    material::lambertian::Lambertian::new(albedo),
                    0.0,
                    1.0,
                )));
            }
        }
    }
    for o in objects.clone() {
        let b = match o.bounding_box(0.0, 1.0) {
            Option::Some(r) => r,
            Option::None => panic!("Cannot find bounding box"),
        };
        println!("before:bounding box {}", b);
    }
    let bvh = BvhNode::new(&mut objects, 0.0, 1.0);
    for o in objects {
        let b = match o.bounding_box(0.0, 1.0) {
            Option::Some(r) => r,
            Option::None => panic!("Cannot find bounding box"),
        };
        println!("after:bounding box {}", b);
    }
}
