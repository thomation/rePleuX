use super::{
    material::{self, random_cosine_direction},
    pdf,
};
use crate::math::{onb, vector};
pub struct CosinePdf {
    uvw: onb::Onb,
}
impl CosinePdf {
    pub fn new(w: &vector::Dir3) -> CosinePdf {
        CosinePdf {
            uvw: onb::Onb::build_from_w(w),
        }
    }
}
impl pdf::Pdf for CosinePdf {
    fn value(&self, dir: &vector::Dir3) -> f64 {
        let cosine = vector::Vec3::dot(&vector::Vec3::unit(dir), self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }

    fn generate(&self) -> vector::Dir3 {
        self.uvw.local(&random_cosine_direction())
    }
}
