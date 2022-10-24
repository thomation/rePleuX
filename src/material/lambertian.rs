use super::material::random_in_half_sphere;
use super::material::{self, random_cosine_direction};
use super::scatter;
use crate::hit::record::HitRecord;
use crate::math::{self, onb, vector};
use crate::texture::texturable;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian<T: texturable::Texturable> {
    albedo: T,
}
impl<T: texturable::Texturable> Lambertian<T> {
    pub fn new(albedo: T) -> Lambertian<T> {
        Lambertian { albedo: albedo }
    }
    pub fn albedo(&self) -> &T {
        &self.albedo
    }
}
impl<T: texturable::Texturable> material::Material for Lambertian<T> {
    fn scatter(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &HitRecord,
    ) -> Option<scatter::ScatterResult> {
        let uvw = onb::Onb::build_from_w(hit_record.normal());
        let mut scatter_dir = uvw.local(&random_cosine_direction());
        scatter_dir.normalize();
        let ray_out = math::ray::Ray::new(
            hit_record.position().clone(),
            scatter_dir.clone(),
            ray_in.time(),
        );
        Option::Some(scatter::ScatterResult::new(
            ray_out,
            self.albedo()
                .value(hit_record.u(), hit_record.v(), hit_record.position()),
            vector::Vec3::dot(uvw.w(), &scatter_dir) / std::f64::consts::PI,
        ))
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
        let cosine = vector::Vec3::dot(hit_record.normal(), scattered.dir());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}
