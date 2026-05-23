use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    ray::{color::Color, ray::Ray},
    surface::{
        material::Material,
        surface::HitRecord,
        texture::Texture,
        textures::solid_color::SolidColorTexture,
    },
    utils::{random_unit_vec, vec_near_zero},
};

pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_from_color(albedo: Color) -> Self {
        Self { texture: Arc::new(SolidColorTexture::new(albedo)) }
    }

    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture,
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut SmallRng,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vec(rng);

        if vec_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.texture.sample(rec.u, rec.v, rec.p);

        true
    }
}