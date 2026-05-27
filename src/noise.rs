use nalgebra::Vector3;
use rand::{rngs::SmallRng, seq::SliceRandom};

use crate::utils::random_f32;

pub struct Perlin {
    randfloat: [f32; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.0; 256];
        let mut rng: SmallRng = rand::make_rng();
        for i in 0..256 {
            randfloat[i] = random_f32(&mut rng, 0.0, 1.0)
        }

        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();

        Perlin { randfloat, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, point: Vector3<f32>) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = point.x.floor() as isize;
        let j = point.y.floor() as isize;
        let k = point.z.floor() as isize;
        let mut c = [[[0.0_f32; 2]; 2]; 2];

        for di in 0..2 { 
            for dj in 0..2 { 
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[
                        self.perm_x[((i + di as isize) & 255) as usize]
                            ^ self.perm_y[((j + dj as isize) & 255) as usize]
                            ^ self.perm_z[((k + dk as isize) & 255) as usize]
        ]}}}
        Perlin::trilinear_interpolation(c, u, v, w)
    }

    pub fn trilinear_interpolation(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum: f32 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u))
                            * (j as f32 * v + (1 - j) as f32 * (1.0 - v))
                            * (k as f32 * w + (1 - k) as f32 * (1.0 - w))
                            * c[i][j][k];
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