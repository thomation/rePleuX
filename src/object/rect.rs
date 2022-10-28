use crate::hit::{aabb, hittable, record};
use crate::material::material;
use crate::math::{random, ray, vector};
use std::sync::Arc;
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<dyn material::Material>,
}
impl XYRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: Arc<dyn material::Material>,
    ) -> XYRect {
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
impl hittable::Hittable for XYRect {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<record::HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.dir().z();
        if t < t_min || t > t_max {
            return Option::None;
        }
        let x = ray.origin().x() + ray.dir().x() * t;
        let y = ray.origin().y() + ray.dir().y() * t;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return Option::None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
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
            self.material.clone(),
        ))
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::hit::aabb::AABB> {
        Option::Some(aabb::AABB::new(
            vector::Point3::new(self.x0, self.y0, self.k - 0.0001),
            vector::Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64 {
        todo!()
    }

    fn random(&self, o: &vector::Point3) -> vector::Dir3 {
        todo!()
    }
}

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn material::Material>,
}
impl XZRect {
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Arc<dyn material::Material>,
    ) -> XZRect {
        XZRect {
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
            material: material,
        }
    }
}
impl hittable::Hittable for XZRect {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hit::record::HitRecord> {
        let t = (self.k - ray.origin().y()) / ray.dir().y();
        if t < t_min || t > t_max {
            return Option::None;
        }
        let x = ray.origin().x() + ray.dir().x() * t;
        let z = ray.origin().z() + ray.dir().z() * t;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = vector::Dir3::new(0.0, 1.0, 0.0);
        let front = vector::Vec3::dot(&outward_normal, &ray.dir()) < 0.0;
        let hit_point = ray.at(t);
        Option::Some(record::HitRecord::new(
            hit_point,
            outward_normal,
            t,
            u,
            v,
            front,
            self.material.clone(),
        ))
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::hit::aabb::AABB> {
        Option::Some(aabb::AABB::new(
            vector::Point3::new(self.x0, self.k - 0.0001, self.z0),
            vector::Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64 {
        match self.hit(&ray::Ray::new(*o, *v, 0.0), 0.00001, std::f64::INFINITY) {
            Some(rec) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let distance_squared = rec.t() * rec.t() * v.length_squared();
                let cosine = vector::Vec3::dot(v, rec.normal()).abs();
                return distance_squared / (cosine * area);
            }
            None => {
                return 0.0;
            }
        }
    }

    fn random(&self, o: &vector::Point3) -> vector::Dir3 {
        let random_point = vector::Point3::new(
            random::generate_range(self.x0, self.x1),
            self.k,
            random::generate_range(self.z0, self.z1),
        );
        random_point - o
    }
}
pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn material::Material>,
}
impl YZRect {
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Arc<dyn material::Material>,
    ) -> YZRect {
        YZRect {
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
            material: material,
        }
    }
}
impl hittable::Hittable for YZRect {
    fn hit(
        &self,
        ray: &crate::math::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hit::record::HitRecord> {
        let t = (self.k - ray.origin().x()) / ray.dir().x();
        if t < t_min || t > t_max {
            return Option::None;
        }
        let y = ray.origin().y() + ray.dir().y() * t;
        let z = ray.origin().z() + ray.dir().z() * t;
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return Option::None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = vector::Dir3::new(1.0, 0.0, 0.0);
        let front = vector::Vec3::dot(&outward_normal, &ray.dir()) < 0.0;
        let hit_point = ray.at(t);
        Option::Some(record::HitRecord::new(
            hit_point,
            outward_normal,
            t,
            u,
            v,
            front,
            self.material.clone(),
        ))
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::hit::aabb::AABB> {
        Option::Some(aabb::AABB::new(
            vector::Point3::new(self.k - 0.0001, self.y0, self.z0),
            vector::Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64 {
        todo!()
    }

    fn random(&self, o: &vector::Point3) -> vector::Dir3 {
        todo!()
    }
}
