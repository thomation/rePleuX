use crate::hit::aabb;
use crate::hit::hittable;
use crate::hit::record;
use crate::material::material;
use crate::math::ray;
use crate::math::vector;
use std::sync::Arc;

pub struct Sphere {
    center0: vector::Point3,
    center1: vector::Point3,
    radius: f64,
    material: Arc<dyn material::Material>,
    time0: f64,
    time1: f64,
}

impl Sphere {
    pub fn new(
        center: vector::Point3,
        radius: f64,
        material: Arc<dyn material::Material>,
    ) -> Sphere {
        Sphere {
            center0: center,
            center1: center,
            radius: radius,
            material: material,
            time0: 0.0,
            time1: 0.0,
        }
    }
    pub fn new_move(
        center0: vector::Point3,
        center1: vector::Point3,
        radius: f64,
        material: Arc<dyn material::Material>,
        time0: f64,
        time1: f64,
    ) -> Sphere {
        Sphere {
            center0: center0,
            center1: center1,
            radius: radius,
            material: material,
            time0: time0,
            time1: time1,
        }
    }
    pub fn center(&self, time: f64) -> vector::Point3 {
        if self.time0 == self.time1 {
            return self.center0;
        }
        let v = (time - self.time0) / (self.time1 - self.time0);
        self.center0 + (self.center1 - self.center0) * v
    }
    pub fn get_sphere_uv(p: &vector::Point3) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + pi;
        let u = phi / (pi * 2.0);
        let v = theta / pi;
        (u, v)
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> std::option::Option<record::HitRecord> {
        let ray_origin = ray.origin().clone();
        let oc = ray_origin - &self.center(ray.time());
        let ray_dir = ray.dir();
        let a = ray_dir.length_squared();
        let half_b = vector::Vec3::dot(&oc, ray_dir);
        let c = vector::Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let mut t = (-half_b - discriminant.sqrt()) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discriminant.sqrt()) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }
        let hit_point = ray.at(t);
        let mut outward_normal = (hit_point.clone() - self.center(ray.time())) / self.radius;
        let front = vector::Vec3::dot(&outward_normal, &ray.dir()) < 0.0;
        outward_normal.normalize();
        let uv = Sphere::get_sphere_uv(&outward_normal);
        Option::Some(record::HitRecord::new(
            hit_point,
            outward_normal,
            t,
            uv.0,
            uv.1,
            front,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<aabb::AABB> {
        let box0 = aabb::AABB::new(
            self.center(time0) - vector::Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + vector::Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = aabb::AABB::new(
            self.center(time1) - vector::Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + vector::Vec3::new(self.radius, self.radius, self.radius),
        );
        Option::Some(aabb::AABB::surrounding_box(box0, box1))
    }

    fn pdf_value(&self, o: &vector::Point3, v: &vector::Dir3) -> f64 {
        todo!()
    }

    fn random(&self, o: &vector::Point3) -> vector::Dir3 {
        todo!()
    }
}
#[test]
fn uv() {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let p1 = vector::Point3::new(1.0, 0.0, 0.0);
    let uv1 = Sphere::get_sphere_uv(&p1);
    assert_eq!(uv1, (0.5, 0.5));
    let p2 = vector::Point3::new(-1.0, 0.0, 0.0);
    let uv2 = Sphere::get_sphere_uv(&p2);
    assert_eq!(uv2, (0.0, 0.5));
    let p3 = vector::Point3::new(0.0, 1.0, 0.0);
    let uv3 = Sphere::get_sphere_uv(&p3);
    assert_eq!(uv3, (0.5, 1.0));
    let p4 = vector::Point3::new(0.0, -1.0, 0.0);
    let uv4 = Sphere::get_sphere_uv(&p4);
    assert_eq!(uv4, (0.5, 0.0));
    let p5 = vector::Point3::new(0.0, 0.0, 1.0);
    let uv5 = Sphere::get_sphere_uv(&p5);
    assert_eq!(uv5, (0.25, 0.50));
    let p6 = vector::Point3::new(0.0, 0.0, -0.1);
    let uv6 = Sphere::get_sphere_uv(&p6);
    assert_eq!(uv6, (0.75, 0.5));
}
