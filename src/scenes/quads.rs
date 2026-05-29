use image::open;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera,
    ray::color::Color,
    surface::{
        materials::{
            lambertian::Lambertian,
        },
        surfaces::{
            quad::Quad,
            surface_group::SurfaceGroup,
        }, textures::image::ImageTexture,
    },
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let left_red = Lambertian::new_from_color(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new_from_color(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new_from_color(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new_from_color(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new_from_color(Color::new(0.2, 0.8, 0.8));

    world.add(Arc::new(Quad::new(
        Vector3::new(-3.0, -2.0, 5.0),
        Vector3::new(0.0, 0.0, -4.0),
        Vector3::new(0.0, 4.0, 0.0),
        Arc::new(left_red)
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(-2.0, -2.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        Arc::new(back_green)
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(3.0, -2.0, 1.0),
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 4.0, 0.0),
        Arc::new(right_blue)
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(-2.0, 3.0, 1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 4.0),
        Arc::new(upper_orange)
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(-2.0, -3.0, 5.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -4.0),
        Arc::new(lower_teal)
    )));

    #[cfg(feature = "denoise")]
    let samples = 20;
    #[cfg(not(feature = "denoise"))]
    let samples = 100;

    let mut camera = Camera::new(
        1.0,
        400,
        samples,
        90.0,
        0.0,
        1.0,
    );

    camera.upd_pos(
        Some(80.0),
        Some(Vector3::new(0.0, 0.0, 9.0)),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(ImageTexture::new(open("textures/sky_1.jpg").expect("Failed to open skybox image, make sure you are in project root!").to_rgb8()));

    (world, camera)
}