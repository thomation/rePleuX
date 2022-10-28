use super::pdf;
use crate::hit::hittable;
use crate::math::vector;
use std::sync::Arc;

pub struct HittablePdf {
    hittable: Arc<dyn hittable::Hittable>,
    origin: vector::Point3,
}
impl HittablePdf {
    pub fn new(hittable: Arc<dyn hittable::Hittable>, origin: vector::Point3) -> HittablePdf {
        HittablePdf { hittable, origin }
    }
}
impl pdf::Pdf for HittablePdf {
    fn value(&self, dir: &vector::Dir3) -> f64 {
        self.hittable.pdf_value(&self.origin, dir)
    }

    fn generate(&self) -> vector::Dir3 {
        self.hittable.random(&self.origin)
    }
}
