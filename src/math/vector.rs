use super::random;

#[derive(Debug, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}
pub type Color3 = Vec3;
pub type Point3 = Vec3;
pub type Dir3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn random() -> Vec3 {
        Vec3 {
            e: [random::generate(), random::generate(), random::generate()],
        }
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random::generate_range(min, max),
                random::generate_range(min, max),
                random::generate_range(min, max),
            ],
        }
    }
    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }
    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalize(&mut self) {
        let u = Vec3::unit(self);
        self.e = u.e;
    }
    pub fn neg(p: &Self) -> Vec3 {
        Vec3{
            e:[-p[0], -p[1], -p[2]]
        }
    }
    pub fn add(p: &Self, q: &Self) -> Vec3 {
        Vec3 {
            e: [p[0] + q[0], p[1] + q[1], p[2] + q[2]]
        }
    }
    pub fn sub(p: &Self, q: &Self) -> Vec3 {
        Vec3 {
            e: [p[0] - q[0], p[1] - q[1], p[2] - q[2]]
        }
    }
    pub fn mul(p: &Self, s: f64) -> Vec3 {
        Vec3 {
            e: [p[0] * s, p[1] * s, p[2] * s],
        }
    }
    pub fn div(p: &Self, s: f64) -> Vec3 {
        Vec3::mul(p, 1.0 / s)
    }
    pub fn dot(p: &Self, q: &Self) -> f64 {
        p.x() * q.x() + p.y() * q.y() + p.z() * q.z()
    }
    pub fn cross(p: &Self, q: &Self) -> Self {
        Vec3 {
            e: [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ],
        }
    }
    pub fn unit(v: &Self) -> Self {
        let l = v.length();
        Vec3 {
            e: [v[0] / l, v[1] / l, v[2] / l],
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }
    pub fn reflect(v: &Self, n: &Self) -> Self {
        let p = Vec3::dot(v, n) * 2.0;
        Vec3::sub(v, &Vec3::mul(n, p))
    }
    pub fn refract(v: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = Vec3::dot(v, n).min(1.0);
        let out_x = Vec3::add(v, &Vec3::mul(n, cos_theta)) * etai_over_etat;
        let out_y = Vec3::mul(n, -(1.0 - out_x.length_squared()).abs().sqrt());
        out_x + out_y
    }
}
impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.e == rhs.e
    }
}
impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}
impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}
impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}
impl std::ops::Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}
impl std::ops::MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self * rhs[0], self * rhs[1], self * rhs[2]],
        }
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}
impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
// The comparing of f64 is not accurate, so the assert may be failed in your computer.
#[test]
fn test_vec3_create() {
    let v = Vec3::new(1.1, 2.1, 3.3);
    assert_eq!(v.x(), 1.1);
    assert_eq!(v.y(), 2.1);
    assert_eq!(v.z(), 3.3);
    assert_eq!(v.length_squared(), 1.1 * 1.1 + 2.1 * 2.1 + 3.3 * 3.3);
    let u = v.clone();
    assert_eq!(u, v);
}
#[test]
fn test_vec3_add() {
    let u = Vec3::new(-1.1, -2.1, -3.3);
    let v = Vec3::new(1.1, 2.1, 3.3);
    let w1 = Vec3::add(&u, &v);
    let w = u + v;
    assert_eq!(w, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(w1, Vec3::new(0.0, 0.0, 0.0));
    let mut x = Vec3::new(1.0, 2.0, 3.0);
    x += Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(x, Vec3::new(1.1, 2.2, 3.3));
}
#[test]
fn test_vec3_sub() {
    let u = Vec3::new(1.1, 2.2, 3.3);
    let v = Vec3::new(0.1, 0.2, 0.3);
    let w1 = Vec3::sub(&u, &v);
    let mut w = u - v;
    assert_eq!(w, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(w1, Vec3::new(1.0, 2.0, 3.0));
    w -= Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(w, Vec3::new(0.9, 1.8, 2.7));
}
#[test]
fn test_vec3_mul() {
    let mut u = Vec3::new(1.1, 2.2, 3.3);
    let u1 = Vec3::mul(&u, 10.0);
    u *= 10.0;
    assert_eq!(u, Vec3::new(11.0, 22.0, 33.0));
    assert_eq!(u1, Vec3::new(11.0, 22.0, 33.0));
    let v = Vec3::new(0.1, 0.2, 0.3);
    let mut w = u * v;
    assert_eq!(w, Vec3::new(1.1, 4.4, 9.9));
    w *= Vec3::new(10.0, 5.0, 1.0);
    assert_eq!(w, Vec3::new(11.0, 22.0, 9.9));
    let x = w * 0.1;
    assert_eq!(x, Vec3::new(1.1, 2.2, 9.9 * 0.1));
    assert_eq!(0.2 * x, Vec3::new(1.1 * 0.2, 2.2 * 0.2, 9.9 * 0.1 * 0.2));
}
#[test]
fn test_vec3_div() {
    let mut u = Vec3::new(1.11, 2.22, 3.33);
    let u1 = Vec3::div(&u, 10.0);
    u /= 10.0;
    assert_eq!(u, Vec3::new(1.11 / 10.0, 2.22 / 10.0, 3.33 / 10.0));
    assert_eq!(u1, Vec3::new(1.11 / 10.0, 2.22 / 10.0, 3.33 / 10.0));
    assert_eq!(u / 2.0, Vec3::new(1.11 / 20.0, 2.22 / 20.0, 3.33 / 20.0));
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
    let u = Vec3::new(1.1, 2.2, 3.3);
    assert_eq!(
        Vec3::unit(&u),
        Vec3::new(1.1 / u.length(), 2.2 / u.length(), 3.3 / u.length())
    );
    assert_eq!(Vec3::unit(&u).length(), 1.0);
}
