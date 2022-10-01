use crate::hit::{self, aabb, hittable, record};
use crate::math::{ray, vector};
use std::sync::Arc;
pub struct RotateY {
    hittable: Arc<dyn hittable::Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<aabb::AABB>,
}
impl RotateY {
    pub fn new(hittable: Arc<dyn hittable::Hittable>, angle: f64) -> RotateY {
        let theta = angle * std::f64::consts::PI / 180.0;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let check = hittable.bounding_box(0.0, 1.0);
        let mut has_bbox = true;
        match check {
            Some(bbox) => {
                let mut min = vector::Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max =
                    vector::Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                            let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                            let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = vector::Point3::new(newx, y, newz);

                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }
                }
                RotateY {
                    hittable: hittable,
                    sin_theta: sin_theta,
                    cos_theta: cos_theta,
                    bbox: Option::Some(aabb::AABB::new(min, max)),
                }
            }
            None => RotateY {
                hittable: hittable,
                sin_theta: sin_theta,
                cos_theta: cos_theta,
                bbox: Option::None,
            },
        }
    }
}
impl hittable::Hittable for RotateY {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<record::HitRecord> {
        let mut origin = ray.origin().clone();
        let mut direction = ray.dir().clone();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.dir()[0] - self.sin_theta * ray.dir()[2];
        direction[2] = self.sin_theta * ray.dir()[0] + self.cos_theta * ray.dir()[2];

        let rotated_ray = ray::Ray::new(origin, direction, ray.time());

        let check = self.hittable.hit(&rotated_ray, t_min, t_max);
        match check {
            Some(rec) => {
                let mut p = rec.position().clone();
                let mut normal = rec.normal().clone();

                p[0] = self.cos_theta * rec.position()[0] + self.sin_theta * rec.position()[2];
                p[2] = -self.sin_theta * rec.position()[0] + self.cos_theta * rec.position()[2];

                normal[0] = self.cos_theta * rec.normal()[0] + self.sin_theta * rec.normal()[2];
                normal[2] = -self.sin_theta * rec.normal()[0] + self.cos_theta * rec.normal()[2];
                let front_face = vector::Vec3::dot(&normal, rotated_ray.dir()) < 0.0;
                Option::Some(record::HitRecord::new(
                    p,
                    normal,
                    rec.t(),
                    rec.u(),
                    rec.v(),
                    front_face,
                    rec.material(),
                ))
            }
            None => Option::None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<aabb::AABB> {
        self.bbox
    }
}
