use super::pdf::{Pdf, PdfValue};
use crate::math::{random, vector};
pub struct MixturePdf {
    p0: PdfValue,
    p1: PdfValue,
}
impl MixturePdf {
    pub fn new(p0: PdfValue, p1: PdfValue) -> Self {
        MixturePdf { p0, p1 }
    }
}
impl Pdf for MixturePdf {
    fn value(&self, dir: &vector::Dir3) -> f64 {
        let mut v0 = 0.0;
        let mut v1 = 0.0;
        match &self.p0 {
            PdfValue::Value(p) => v0 = p.value(dir),
            PdfValue::Null => v0 = 0.0,
        }
        match &self.p1 {
            PdfValue::Value(p) => v1 = p.value(dir),
            PdfValue::Null => v1 = 0.0,
        }
        0.5 * v0 + 0.5 * v1 
    }

    fn generate(&self) -> vector::Dir3 {
        let mut p;
        if random::generate() < 0.5 {
            p = &self.p0;
        } else {
            p = &self.p1;
        }
        match p {
            PdfValue::Value(p) => p.generate(),
            PdfValue::Null => vector::Dir3::new(1.0, 0.0, 0.0),
        }
    }
}
