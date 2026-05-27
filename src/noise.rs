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
    let i = ((4.0 * point.x) as isize & 255) as usize;
    let j = ((4.0 * point.y) as isize & 255) as usize;
    let k = ((4.0 * point.z) as isize & 255) as usize;
        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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