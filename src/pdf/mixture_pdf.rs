use super::pdf::{Pdf, PdfNode};
use crate::math::{random, vector};
pub struct MixturePdf {
    p0: PdfNode,
    p1: PdfNode,
}
impl MixturePdf {
    pub fn new(p0: PdfNode, p1: PdfNode) -> Self {
        MixturePdf { p0, p1 }
    }
}
impl Pdf for MixturePdf {
    fn value(&self, dir: &vector::Dir3) -> f64 {
        let mut v0 = 0.0;
        let mut v1 = 0.0;
        match &self.p0 {
            PdfNode::Node(p) => v0 = p.value(dir),
            PdfNode::Null => v0 = 0.0,
        }
        match &self.p1 {
            PdfNode::Node(p) => v1 = p.value(dir),
            PdfNode::Null => v1 = 0.0,
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
            PdfNode::Node(p) => p.generate(),
            PdfNode::Null => vector::Dir3::new(1.0, 0.0, 0.0),
        }
    }
}
