use crate::{ray::{color::Color, ray::Ray}, surface::{material::Material, surface::HitRecord}, utils::{reflect, refract}};

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri;
        if rec.front_face { ri = 1.0 / self.refraction_index; }
        else { ri = self.refraction_index; }

        let unit_dir = r_in.direction().normalize();

        let cos_theta = -unit_dir.dot(&rec.normal).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);

        let can_refract  = !(ri* sin_theta > 1.0);
        let dir;

        if can_refract {
            dir = refract(unit_dir, rec.normal, ri);
        } else {
            dir = reflect(unit_dir, rec.normal);
        }

        *scattered = Ray::new(rec.p, dir);
        true
    }
}