use super::material;
use super::scatter;
use crate::hit::record::HitRecord;
use crate::math;
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
        let mut scatter_dir = hit_record.normal().clone() + material::random_in_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal().clone();
        }
        let ray_out =
            math::ray::Ray::new(hit_record.position().clone(), scatter_dir, ray_in.time());
        Option::Some(scatter::ScatterResult::new(
            ray_out,
            self.albedo()
                .value(hit_record.u(), hit_record.v(), hit_record.position()),
        ))
    }
    fn emitted(&self, u: f64, v: f64, p: &math::vector::Point3) -> math::vector::Color3 {
        math::vector::Color3::zero()
    }
}
