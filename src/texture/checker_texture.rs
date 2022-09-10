use super::texturable;
pub struct CheckerTexture<T: texturable::Texturable, S: texturable::Texturable> {
    even: T,
    odd: S,
}
impl<T: texturable::Texturable, S: texturable::Texturable> CheckerTexture<T, S> {
    pub fn new(even: T, odd: S) -> Self {
        CheckerTexture {
            even,
            odd,
        }
    }
}
impl<T: texturable::Texturable, S: texturable::Texturable> texturable::Texturable
    for CheckerTexture<T, S>
{
    fn value(
        &self,
        u: f64,
        v: f64,
        p: &crate::math::vector::Point3,
    ) -> crate::math::vector::Color3 {
        let sines = (p.x() * 10.0).sin() * (p.y() * 10.0).sin() * (p.z() * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
