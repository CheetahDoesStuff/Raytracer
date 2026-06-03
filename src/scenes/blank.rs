use image::open;
use nalgebra::Vector3;

use crate::{camera::Camera, surface::{surfaces::surface_group::SurfaceGroup, textures::image::ImageTexture}};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

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