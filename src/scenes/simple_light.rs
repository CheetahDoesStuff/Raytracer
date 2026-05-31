use image::open;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera, ray::color::Color, surface::{
        material::Material, materials::{diffuse_light::DiffuseLight, lambertian::Lambertian}, surfaces::{
            sphere::Sphere,
            surface_group::SurfaceGroup,
        }, texture::Texture, textures::{image::ImageTexture, noise::NoiseTexture, solid_color::SolidColorTexture}
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

    world.add(Arc::new(Quad));


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
        Some(Vector3::new(13.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(ImageTexture::new(open("textures/sky_1.jpg").expect("Failed to open skybox image, make sure you are in project root!").to_rgb8()));

    (world, camera)
}