use super::{random, vector};
const POINT_COUNT: usize = 256;
#[derive(Debug, Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat = vec![];
        for _ in 0..POINT_COUNT {
            ranfloat.push(random::generate());
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Perlin {
            ranfloat,
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
        for i in 0..POINT_COUNT {
            print!("{}, ", p[i]);
        }
        // println!("p is permuted");
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
        let i = (4.0 * p.x()) as i64 & 255;
        let j = (4.0 * p.y()) as i64 & 255;
        let k = (4.0 * p.z()) as i64 & 255;
        // println!("i = {}, j = {}, k = {}, p:{}", i, j, k, p);

        self.ranfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
}
