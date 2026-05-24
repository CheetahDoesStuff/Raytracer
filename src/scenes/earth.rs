use image::open;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera,
    surface::{
        materials::{
            lambertian::Lambertian,
        }, surfaces::{
            sphere::Sphere,
            surface_group::SurfaceGroup,
        }, texture::Texture, textures::image::ImageTexture
    },
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let earth: Arc<dyn Texture> = Arc::new(ImageTexture::new(
        open("textures/blue_marble.jpg").expect("Failed to open earth texture, make sure you are in project root!").to_rgb8()
    ));

    let moon: Arc<dyn Texture> = Arc::new(ImageTexture::new(
        open("textures/moon.png").expect("Failed to open moon texture, make sure you are in project root!").to_rgb8()
    ));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, 0.25),
        2.0,
        Arc::new(Lambertian::new(earth.clone()))
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(-0.5, 1.0, -2.25),
        0.54,
        Arc::new(Lambertian::new(moon.clone()))
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
        Some(Vector3::new(13.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(ImageTexture::new(open("textures/sky_1.jpg").expect("Failed to open skybox image, make sure you are in project root!").to_rgb8()));

    (world, camera)
}