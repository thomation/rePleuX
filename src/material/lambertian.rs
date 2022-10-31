use super::material;
use super::scatter;
use crate::hit::record::HitRecord;
use crate::math::{self, onb, random, ray, vector};
use crate::pdf::{cosine_pdf, pdf};
use crate::texture::texturable;
use std::sync::Arc;

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
        Option::Some(scatter::ScatterResult::new(
            self.albedo()
                .value(hit_record.u(), hit_record.v(), hit_record.position()),
            pdf::PdfValue::Value(Arc::new(cosine_pdf::CosinePdf::new(hit_record.normal()))),
            scatter::SpecularValue::Null,
        ))
    }
    fn emitted(
        &self,
        ray_in: &math::ray::Ray,
        hit_record: &HitRecord,
        u: f64,
        v: f64,
        p: &math::vector::Point3,
    ) -> math::vector::Color3 {
        math::vector::Color3::zero()
    }

    fn scatting_pdf(
        &self,
        ray_in: &ray::Ray,
        hit_record: &HitRecord,
        scattered: &math::ray::Ray,
    ) -> f64 {
        let cosine = vector::Vec3::dot(hit_record.normal(), scattered.dir());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}
