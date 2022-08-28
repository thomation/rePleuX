use crate::math::ray;
use crate::math::vector;
struct AABB {
    min: vector::Point3,
    max: vector::Point3,
}
impl AABB {
    pub fn new(min: vector::Point3, max: vector::Point3) -> AABB {
        AABB { min: min, max: max }
    }
    pub fn hit(&self, ray: ray::Ray, t_min: f64, t_max: f64) -> bool {
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
}
