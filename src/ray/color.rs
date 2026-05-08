use nalgebra::Vector3;

use crate::interval::Interval;

pub type Color = Vector3<f32>;

pub fn write_col(col: &Color) {
    let r = col.x;
    let g = col.y;
    let b = col.z;

    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    println!("{} {} {}", ir, ig, ib);
}
