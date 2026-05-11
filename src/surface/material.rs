use crate::{
    ray::{color::Color, ray::Ray},
    surface::surface::HitRecord,
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
