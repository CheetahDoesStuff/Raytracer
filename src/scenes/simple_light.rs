use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera, ray::color::Color, surface::{
        material::Material, materials::{diffuse_light::DiffuseLight, lambertian::Lambertian}, surfaces::{
            quad::Quad, sphere::Sphere, surface_group::SurfaceGroup
        }, textures::{noise::NoiseTexture, solid_color::SolidColorTexture}
    }
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let noise_mat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0))));
    let light_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColorTexture::new(Color::new(0.4, 0.4, 0.4)))));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.25),
        1000.0,
        noise_mat.clone()
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        noise_mat.clone()
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(3.0, 1.0, -2.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
        light_mat
    )));


    #[cfg(feature = "denoise")]
    let samples = 20;
    #[cfg(not(feature = "denoise"))]
    let samples = 100;

    let mut camera = Camera::new(
        16.0 / 9.0,
        1920,
        samples,
        90.0,
        0.0,
        17.0,
    );

    camera.upd_pos(
        Some(20.0),
        Some(Vector3::new(26.0, 3.0, 6.0)),
        Some(Vector3::new(0.0, 2.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(SolidColorTexture::new(Color::new(0.0, 0.0, 0.0)));

    (world, camera)
}