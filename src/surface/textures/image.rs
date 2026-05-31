use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::Vector3;

use crate::{ray::color::Color, surface::texture::Texture};

pub struct ImageTexture {
    texture: RgbImage
}

impl ImageTexture {
    pub fn new(texture: RgbImage) -> Self {
        ImageTexture { texture: texture }
    }

    pub fn empty() -> Self {
        let white = Rgb([255, 255, 255]);
        let img = ImageBuffer::from_pixel(1, 1, white);
        ImageTexture { texture: img }
    }
}

impl Texture for ImageTexture {
    fn sample(&self, u: &f32, v: &f32, _pos: &Vector3<f32>) -> Color {
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