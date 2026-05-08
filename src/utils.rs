use core::f32;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}