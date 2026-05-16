use rand::rngs::SmallRng;

use crate::{
    ray::{color::Color, ray::Ray},
    surface::{material::Material, surface::HitRecord},
    utils::{random_f32, reflect, refract},
};

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(self: &Self, cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &mut SmallRng) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_dir = r_in.direction().normalize();
        let cos_theta = -unit_dir.dot(&rec.normal).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let can_refract = !(ri * sin_theta > 1.0);
        let dir = if can_refract || (self.reflectance(cos_theta, ri) > random_f32(rng, 0.0, 1.0)) {
            refract(unit_dir, rec.normal, ri)
        } else {
            reflect(unit_dir, rec.normal)
        };
        *scattered = Ray::new(rec.p, dir);
        true
    }
}