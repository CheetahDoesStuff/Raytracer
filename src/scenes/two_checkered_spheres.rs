use image::open;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera,
    ray::color::Color,
    surface::{
        materials::{
            lambertian::Lambertian,
        }, surfaces::{
            sphere::Sphere,
            surface_group::SurfaceGroup,
        }, texture::Texture, textures::{checkered::CheckeredTexture, image::ImageTexture}
    },
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let tex: Arc<dyn Texture> = Arc::new(CheckeredTexture::new(
        0.32,
        Color::new(0.48, 0.31, 0.6),
        Color::new(0.9, 0.9, 0.9)
    ));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(tex.clone()))
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(tex.clone()))
    )));

    #[cfg(feature = "denoise")]
    let samples = 20;
    #[cfg(not(feature = "denoise"))]
    let samples = 100;

    let mut camera = Camera::new(
        16.0 / 9.0,
        400,
        samples,
        90.0,
        0.0,
        17.0,
    );

    camera.upd_pos(
        Some(20.0),
        Some(Vector3::new(13.0, 2.0, 3.0)),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(ImageTexture::new(open("textures/sky_1.jpg").expect("Failed to open skybox image, make sure you are in project root!").to_rgb8()));

    (world, camera)
}