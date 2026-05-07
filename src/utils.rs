use core::f32;

const infinity: f32 = f32::INFINITY;
const pi: f32 = 3.1415926535897932385;

fn double_degrees_to_radians(degrees: f32) -> f32 {
    return degrees * pi / 180.0;
}