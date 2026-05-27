use nalgebra::Vector3;
use rand::{rngs::SmallRng, seq::SliceRandom};

use crate::utils::random_unit_vec;

pub struct Perlin {
    randvec: [Vector3<f32>; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vector3::new(0.0, 0.0, 0.0); 256];
        let mut rng: SmallRng = rand::make_rng();
        for i in 0..256 {
            randvec[i] = random_unit_vec(&mut rng)
        }

        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();

        Perlin { randvec, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, point: Vector3<f32>) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as isize;
        let j = point.y.floor() as isize;
        let k = point.z.floor() as isize;
        let mut c = [[[Vector3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 { 
            for dj in 0..2 { 
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[
                        self.perm_x[((i + di as isize) & 255) as usize]
                            ^ self.perm_y[((j + dj as isize) & 255) as usize]
                            ^ self.perm_z[((k + dk as isize) & 255) as usize]
        ]}}}
        Perlin::perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, point: Vector3<f32>, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut mut_point = point.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(mut_point);
            weight *= 0.5;
            mut_point *= 2.0;
        }

        accum.abs()
    }

    pub fn perlin_interpolation(c: [[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vector3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                            * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                            * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                            * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }

    pub fn generate_perm() -> [usize; 256] {
        let mut p = [0usize; 256];
        for i in 0..256 {
            p[i] = i;
        }

        let mut rng: SmallRng = rand::make_rng();
        (&mut p).shuffle(&mut rng);
        p
    }
}