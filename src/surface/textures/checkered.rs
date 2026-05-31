use std::sync::Arc;

use crate::{ray::color::Color, surface::{texture::Texture, textures::solid_color::SolidColorTexture}};


pub struct CheckeredTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}


impl CheckeredTexture {
    pub fn new(scale: f32, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColorTexture::new(even)),
            odd: Arc::new(SolidColorTexture::new(odd)),
        }
    }

    pub fn new_from_textures(
        scale: f32,
        even: Arc<dyn Texture>,
        odd: Arc<dyn Texture>,
    ) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for CheckeredTexture {
    fn sample(&self, u: &f32, v: &f32, pos: &nalgebra::Vector3<f32>) -> Color {
        let x_int = (self.inv_scale * pos.x).floor() as i32;
        let y_int = (self.inv_scale * pos.y).floor() as i32;
        let z_int = (self.inv_scale * pos.z).floor() as i32;

        let even = (x_int + y_int + z_int) % 2 == 0;
        if even { return self.even.sample(u, v, pos); }
        else { return self.odd.sample(u, v, pos); }
    }
}