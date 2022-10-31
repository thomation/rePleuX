use super::{material, scatter};
use crate::math::{random, ray, vector};
use crate::pdf::pdf;
use crate::texture::texturable;
pub struct Isotropic<T: texturable::Texturable> {
    albedo: T,
}
impl<T: texturable::Texturable> Isotropic<T> {
    pub fn new(albedo: T) -> Isotropic<T> {
        Isotropic { albedo }
    }
}
impl<T: texturable::Texturable> material::Material for Isotropic<T> {
    fn scatter(
        &self,
        ray_in: &ray::Ray,
        hit_record: &crate::hit::record::HitRecord,
    ) -> Option<scatter::ScatterResult> {
        Option::Some(scatter::ScatterResult::new(
            self.albedo
                .value(hit_record.u(), hit_record.v(), hit_record.position()),
            pdf::PdfValue::Null,
            scatter::SpecularValue::Value(ray::Ray::new(
                hit_record.position().clone(),
                random::random_in_unit_sphere(),
                ray_in.time(),
            )),
        ))
    }

    fn emitted(
        &self,
        ray_in: &ray::Ray,
        hit_record: &crate::hit::record::HitRecord,
        u: f64,
        v: f64,
        p: &crate::math::vector::Point3,
    ) -> vector::Color3 {
        vector::Color3::zero()
    }

    fn scatting_pdf(
        &self,
        ray_in: &ray::Ray,
        hit_record: &crate::hit::record::HitRecord,
        scattered: &ray::Ray,
    ) -> f64 {
        todo!()
    }
}
