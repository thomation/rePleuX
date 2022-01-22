pub struct Vec3
{
    e:(f64, f64, f64)
}
impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3
    {
        Vec3{e:(x, y, z)}
    }
    pub fn x(&self) -> f64
    {
        self.e.0
    }
    pub fn y(&self) -> f64
    {
        self.e.1
    }
    pub fn z(&self) -> f64
    {
        self.e.2
    }
    pub fn length_squared(&self) -> f64
    {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }
    pub fn length(&self) -> f64
    {
        self.length_squared().sqrt()
    }
}