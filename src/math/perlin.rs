use super::{random, vector};
const POINT_COUNT: usize = 256;
#[derive(Debug, Clone)]
pub struct Perlin {
    ranvec: Vec<vector::Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = vec![];
        for _ in 0..POINT_COUNT {
            let mut v = vector::Vec3::random_range(-1.0, 1.0);
            v.normalize();
            ranvec.push(v);
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![];
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }
    fn permute(p: &mut Vec<usize>, n: usize) {
        let mut i = n - 1;
        while i > 0 {
            let target = random::generate_range_int(0, i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
            i -= 1;
        }
    }
    pub fn noise(&self, p: &vector::Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;
        let mut c: [[[vector::Vec3; 2]; 2]; 2] = [
            [
                [vector::Vec3::zero(), vector::Vec3::zero()],
                [vector::Vec3::zero(), vector::Vec3::zero()],
            ],
            [
                [vector::Vec3::zero(), vector::Vec3::zero()],
                [vector::Vec3::zero(), vector::Vec3::zero()],
            ],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]];
                }
            }
        }
        Perlin::trilinear_interp(&c, u, v, w)
    }
    fn trilinear_interp(c: &[[[vector::Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = vector::Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * vector::Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }

        return accum;
    }
}
