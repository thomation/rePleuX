use crate::math::vector;
use std::sync::Arc;
pub trait Pdf {
    fn value(&self, dir: &vector::Dir3) -> f64;
    fn generate(&self) -> vector::Dir3;
}


#[derive(Clone)]
pub enum PdfValue
{
    Value(Arc<dyn Pdf>),
    Null,
}