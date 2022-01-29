#[derive(Debug)]
pub struct Vec3 {
    e: (f64, f64, f64),
}
pub type Color3 = Vec3;
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: (x, y, z) }
    }
    pub fn x(&self) -> f64 {
        self.e.0
    }
    pub fn y(&self) -> f64 {
        self.e.1
    }
    pub fn z(&self) -> f64 {
        self.e.2
    }
    pub fn length_squared(&self) -> f64 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn dot(p: &Self, q: &Self) -> f64 {
        p.x() * q.x() + p.y() * q.y() + p.z() * q.z()
    }
    pub fn cross(p: &Self, q: &Self) -> Self {
        Vec3 {
            e: (
                p.e.1 * q.e.2 - p.e.2 * q.e.1,
                p.e.2 * q.e.0 - p.e.0 * q.e.2,
                p.e.0 * q.e.1 - p.e.1 * q.e.0,
            ),
        }
    }
}
impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.e == rhs.e
    }
}
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
        }
    }
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e.0 += rhs.e.0;
        self.e.1 += rhs.e.1;
        self.e.2 += rhs.e.2;
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
        }
    }
}
impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e.0 -= rhs.e.0;
        self.e.1 -= rhs.e.1;
        self.e.2 -= rhs.e.2;
    }
}
impl std::ops::Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            e: (self.e.0 * rhs.e.0, self.e.1 * rhs.e.1, self.e.2 * rhs.e.2),
        }
    }
}
impl std::ops::MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e.0 *= rhs.e.0;
        self.e.1 *= rhs.e.1;
        self.e.2 *= rhs.e.2;
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            e: (self.e.0 * rhs, self.e.1 * rhs, self.e.2 * rhs),
        }
    }
}
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e.0 *= rhs;
        self.e.1 *= rhs;
        self.e.2 *= rhs;
    }
}

#[test]
fn test_vec3_create() {
    let v = Vec3::new(1.1, 2.1, 3.3);
    assert_eq!(v.x(), 1.1);
    assert_eq!(v.y(), 2.1);
    assert_eq!(v.z(), 3.3);
    assert_eq!(v.length_squared(), 1.1 * 1.1 + 2.1 * 2.1 + 3.3 * 3.3);
}
#[test]
fn test_vec3_add() {
    let u = Vec3::new(-1.1, -2.1, -3.3);
    let v = Vec3::new(1.1, 2.1, 3.3);
    let w = u + v;
    assert_eq!(w, Vec3::new(0.0, 0.0, 0.0));
    let mut x = Vec3::new(1.0, 2.0, 3.0);
    x += Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(x, Vec3::new(1.1, 2.2, 3.3));
}
#[test]
fn test_vec3_sub() {
    let u = Vec3::new(1.1, 2.2, 3.3);
    let v = Vec3::new(0.1, 0.2, 0.3);
    let mut w = u - v;
    assert_eq!(w, Vec3::new(1.0, 2.0, 3.0));
    w -= Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(w, Vec3::new(0.9, 1.8, 2.7));
}
#[test]
fn test_vec3_mul() {
    let mut u = Vec3::new (1.1, 2.2, 3.3);
    u *= 10.0;
    assert_eq!(u, Vec3::new(11.0, 22.0, 33.0));
    let v = Vec3::new (0.1, 0.2, 0.3);
    let mut w = u * v;
    assert_eq!(w, Vec3::new(1.1, 4.4, 9.9));
    w *= Vec3::new(10.0, 5.0, 1.0);
    assert_eq!(w, Vec3::new(11.0, 22.0, 9.9));
    let x = w * 0.1;
    assert_eq!(x, Vec3::new(1.1, 2.2, 9.9 * 0.1));
}
#[test]
fn test_vec3_complex() {
    let i = Vec3::new(1.0, 0.0, 0.0);
    let j = Vec3::new(0.0, 1.0, 0.0);
    let k = Vec3::cross(&i, &j);
    assert_eq!(k, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(
        Vec3::dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(1.0, 2.0, 3.0)),
        14.0
    );
}
