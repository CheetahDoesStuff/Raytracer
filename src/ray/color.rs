use nalgebra::Vector3;

pub type Color = Vector3<f32>;

pub fn write_col(col: &Color) {
    let r = col.x;
    let g = col.y;
    let b = col.z;

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    println!("{} {} {}", ir, ig, ib);
}