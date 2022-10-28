use crate::hit::{aabb, hittable, record};
use crate::material::material;
use crate::math::{random, vector};
use std::sync::Arc;
pub struct ConstantMedium {
    boundary: Arc<dyn hittable::Hittable>,
    phase_function: Arc<dyn material::Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn hittable::Hittable>,
        d: f64,
        phase_function: Arc<dyn material::Material>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / d,
        }
    }
}

impl hittable::Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &crate::math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<record::HitRecord> {
        match self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY) {
            Some(rec1) => match self.boundary.hit(ray, rec1.t() + 0.0001, f64::INFINITY) {
                Some(rec2) => {
                    let mut t1 = if rec1.t() < t_min { t_min } else { rec1.t() };
                    let t2 = if rec2.t() > t_max { t_max } else { rec2.t() };
                    if t1 >= t2 {
                        return Option::None;
                    }
                    if t1 < 0.0 {
                        t1 = 0.0;
                    }
                    let ray_length = ray.dir().length();
                    let distance_inside_boundary = (t2 - t1) * ray_length;
                    let hit_distance = self.neg_inv_density * random::generate().ln();

                    if hit_distance > distance_inside_boundary {
                        return Option::None;
                    }
                    let t = t1 + hit_distance / ray_length;
                    let p = ray.at(t);

                    let outward_normal = vector::Dir3::new(1.0, 0.0, 0.0); // arbitrary
                    let front_face = true; // also arbitrary
                    Option::Some(record::HitRecord::new(
                        p,
                        outward_normal,
                        t,
                        0.0,
                        0.0,
                        front_face,
                        self.phase_function.clone(),
                    ))
                }
                None => Option::None,
            },
            None => Option::None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<aabb::AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64 {
        todo!()
    }

    fn random(&self, o: &vector::Point3) -> vector::Dir3 {
        todo!()
    }
}
