use crate::{
    ray::{color::Color, ray::Ray},
    surface::surface::HitRecord,
    utils::{random_unit_vec, reflect, refract, vec_near_zero},
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vec();

        if vec_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

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