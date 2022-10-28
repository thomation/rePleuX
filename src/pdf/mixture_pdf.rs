use super::pdf::Pdf;
use crate::math::{random, vector};
pub struct MixturePdf<T: Pdf, S: Pdf> {
    p0: T,
    p1: S,
}
impl<T: Pdf, S: Pdf> MixturePdf<T, S> {
    pub fn new(p0: T, p1: S) -> Self {
        MixturePdf { p0, p1 }
    }
}
impl<T:Pdf, S:Pdf> Pdf for MixturePdf<T, S> {
    fn value(&self, dir: &vector::Dir3) -> f64 {
        0.5 * self.p0.value(dir) + 0.5 * self.p1.value(dir)
    }

    fn generate(&self) -> vector::Dir3 {
        if random::generate() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }

    }
}
