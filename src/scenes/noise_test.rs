use image::open;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera,
    surface::{
        material::Material,
        materials::lambertian::Lambertian,
        surfaces::{
            sphere::Sphere,
            surface_group::SurfaceGroup,
        },
        texture::Texture,
        textures::{
            image::ImageTexture,
            noise::NoiseTexture,
        },
    },
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let texture: Arc<dyn Texture> =
        Arc::new(NoiseTexture::new(4.0));

    let material: Arc<dyn Material> =
        Arc::new(Lambertian::new(texture));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        material.clone(),
    )));

    #[cfg(feature = "denoise")]
    let samples = 20;
    #[cfg(not(feature = "denoise"))]
    let samples = 100;

    let mut camera = Camera::new(
        16.0 / 9.0,
        400,
        samples,
        20.0,
        0.0,
        50.0,
    );

    camera.upd_pos(
        Some(20.0),
        Some(Vector3::new(13.0, 2.0, 3.0)),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(
        ImageTexture::new(
            open("textures/sky_1.jpg")
                .expect("Failed to open skybox image")
                .to_rgb8(),
        ),
    );

    (world, camera)
}