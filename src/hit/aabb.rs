use crate::math::ray;
use crate::math::vector;
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: vector::Point3,
    max: vector::Point3,
}
impl AABB {
    pub fn new(min: vector::Point3, max: vector::Point3) -> AABB {
        AABB { min: min, max: max }
    }
    pub fn min(&self) -> &vector::Point3 {
        &self.min
    }
    pub fn max(&self) -> &vector::Point3 {
        &self.max
    }
    pub fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> bool {
        let mut tn = t_min;
        let mut tx = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            if t0 > tn {
                tn = t0;
            }
            if t1 < tx {
                tx = t1;
            }
            if tx <= tn {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = vector::Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let big = vector::Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );
        AABB::new(small, big)
    }
}
impl std::fmt::Display for AABB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min(), self.max())
    }
}
