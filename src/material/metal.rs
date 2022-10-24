use super::material;
use super::scatter;
use crate::hit::record::HitRecord;
use crate::math;
use crate::math::vector;
use crate::math::vector::Vec3;

pub struct Metal {
    albedo: vector::Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: vector::Color3, fuzz: f64) -> Self {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
    pub fn albedo(&self) -> &vector::Color3 {
        &self.albedo
    }
    pub fn fuzz(&self) -> f64 {
        self.fuzz
    }
}

impl material::Material for Metal {
    fn scatter(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &HitRecord,
    ) -> std::option::Option<scatter::ScatterResult> {
        let ray_dir = Vec3::unit(&ray_in.dir());
        let reflected = ray_dir.reflect(hit_record.normal().clone());
        if Vec3::dot(&reflected, hit_record.normal()) > 0.0 {
            let scattered = math::ray::Ray::new(
                hit_record.position().clone(),
                reflected + material::random_in_unit_sphere() * self.fuzz(),
                ray_in.time(),
            );
            Option::Some(scatter::ScatterResult::new(
                scattered,
                self.albedo().clone(),
                0.0,
            ))
        } else {
            Option::None
        }
    }
    fn emitted(
        &self,
        hit_record: &HitRecord,
        u: f64,
        v: f64,
        p: &math::vector::Point3,
    ) -> math::vector::Color3 {
        math::vector::Color3::zero()
    }

    fn scatting_pdf(&self, hit_record: &HitRecord, scattered: &math::ray::Ray) -> f64 {
        todo!()
    }
}
