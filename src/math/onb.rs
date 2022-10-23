use super::vector;
pub struct Onb {
    axis: [vector::Dir3; 3],
}
impl Onb {
    pub fn build_from_w(n: &vector::Dir3) -> Onb {
        let w = vector::Vec3::unit(n);
        let a = if w.x().abs() > 0.9 {
            vector::Dir3::new(0.0, 1.0, 0.0)
        } else {
            vector::Dir3::new(1.0, 0.0, 0.0)
        };
        let mut v = vector::Vec3::cross(&w, &a);
        v.normalize();
        let u = vector::Vec3::cross(&w, &v);
        Onb { axis: [u, v, w] }
    }
    pub fn local(&self, a: &vector::Dir3) -> vector::Dir3{
        self.u().clone() * a.x() + self.v().clone() * a.y() + self.w().clone() * a.z()
    }
    fn u(&self) -> &vector::Vec3 {
        &self.axis[0]
    }
    fn v(&self) -> &vector::Vec3 {
        &self.axis[1]
    }
    pub fn w(&self) -> &vector::Vec3 {
        &self.axis[2]
    }
}
