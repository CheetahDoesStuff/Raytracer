use nalgebra::Vector3;

use crate::interval::Interval;

pub type Color = Vector3<f32>;

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return f32::sqrt(linear_component);
    }
    0.0
}

pub fn write_col(col: &Color) {
    let r = linear_to_gamma(col.x);
    let g = linear_to_gamma(col.y);
    let b = linear_to_gamma(col.z);

    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    println!("{} {} {}", ir, ig, ib);
}

pub fn write_col_string(col: &Color) -> String {
    let r = linear_to_gamma(col.x);
    let g = linear_to_gamma(col.y);
    let b = linear_to_gamma(col.z);

    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    format!("{} {} {}\n", ir, ig, ib)
}
