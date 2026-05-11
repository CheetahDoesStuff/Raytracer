use crate::{
    ray::{color::Color, ray::Ray},
    surface::{material::Material, surface::HitRecord},
    utils::{random_unit_vec, reflect},
};

pub struct Metal {
    pub albedo: Color,
    pub matte: f32,
}

impl Metal {
    pub fn new(albedo: Color, matte: f32) -> Self {
        Self { albedo, matte }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = reflected + (self.matte * random_unit_vec());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
