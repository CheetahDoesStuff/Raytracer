use image::{ImageBuffer, Rgb, RgbImage};

use crate::ray::color::Color;

pub struct Texture {
    texture: RgbImage
}

impl Texture {
    pub fn new(texture: RgbImage) -> Self {
        Texture { texture: texture }
    }

    pub fn empty(w: u32, h: u32) -> Self {
        let white = Rgb([255, 255, 255]);
        let img = ImageBuffer::from_pixel(w, h, white);
        Texture { texture: img }
    }

    pub fn sample_texture(&self, u: f32, v: f32) -> Color {
        let w = self.texture.width();
        let h = self.texture.height();

        let x = ((u * w as f32) as u32) % w;
        let y = ((v * h as f32) as u32) % h;

        let rgb = self.texture.get_pixel(x, y);
        Color::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0
        )
    }
}