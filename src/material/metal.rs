use super::material;
use super::scatter;
use crate::hit::record::HitRecord;
use crate::math;
use crate::math::vector;
use crate::math::vector::Vec3;

pub struct Metal {
    albedo: vector::Color3,
}

impl Metal {
    fn new(albedo: vector::Color3) -> Self {
        Metal { albedo: albedo }
    }
    pub fn albedo(&self) -> vector::Color3 {
        self.albedo.clone()
    }
}

impl material::Material for Metal {
    fn scatter(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &HitRecord,
    ) -> std::option::Option<scatter::ScatterResult> {
        let ray_dir = Vec3::unit(&ray_in.dir());
        let normal = hit_record.normal();
        let reflected = Vec3::reflect(&ray_dir, &normal);
        if Vec3::dot(&reflected, &normal) > 0.0 {
            let scattered = math::ray::Ray::new(hit_record.position(), reflected);
            Option::Some(scatter::ScatterResult::new(scattered, self.albedo()))
        } else {
            Option::None
        }
    }
}
