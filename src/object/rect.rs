use crate::hit::aabb;
use crate::hit::hittable;
use crate::hit::record;
use crate::material::material;
use crate::math::vector;
pub struct XYRect<M: material::Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: M,
}
impl<M: material::Material> XYRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: M) -> XYRect<M> {
        XYRect {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
            material: material,
        }
    }
}
impl<M: material::Material + std::marker::Send + std::marker::Sync> hittable::Hittable
    for XYRect<M>
{
    fn hit(
        &self,
        ray: &crate::math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hit::record::HitRecord> {
        let t =(self.k - ray.origin().z()) / ray.dir().z();
    if t < t_min || t > t_max {
        return Option::None;
    }
    let x = ray.origin().x() + ray.dir().x() * t;
    let y = ray.origin().y() + ray.dir().y() * t;
    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
        return Option::None;
    }
    let u = (x-self.x0)/(self.x1-self.x0);
    let v = (y-self.y0)/(self.y1-self.y0);
    let outward_normal = vector::Dir3::new(0.0, 0.0, 1.0);
    let front = vector::Vec3::dot(&outward_normal, &ray.dir()) < 0.0;
    let hit_point = ray.at(t);
        Option::Some(record::HitRecord::new(
            hit_point,
            outward_normal,
            t,
            u,
            v,
            front,
            &self.material,
        ))
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::hit::aabb::AABB> {
        Option::Some(aabb::AABB::new(
            vector::Point3::new(self.x0, self.y0, self.k - 0.001),
            vector::Point3::new(self.x1, self.y1, self.k + 0.001),
        ))
    }
}
