use crate::texture::texturable;
use super::material;

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight<T: texturable::Texturable> {
    emit: T,
}
impl <T:texturable::Texturable> DiffuseLight<T> {
    pub fn new(emit: T) -> DiffuseLight<T> {
        DiffuseLight {
            emit: emit, 
        }
    }
}
impl <T: texturable::Texturable> material::Material for DiffuseLight<T> {
    fn scatter(&self, ray_in: &crate::math::ray::Ray, hit_record: &crate::hit::record::HitRecord) -> Option<super::scatter::ScatterResult> {
        Option::None
    }

    fn emitted(&self, u: f64, v: f64, p: &crate::math::vector::Point3) -> crate::math::vector::Color3 {
        self.emit.value(u, v, p)
    }
}