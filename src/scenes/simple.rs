use image::open;
use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera,
    ray::color::Color,
    surface::{
        material::Material,
        materials::{
            dielectric::Dielectric,
            lambertian::Lambertian,
            metal::Metal,
        },
        surfaces::{
            sphere::Sphere,
            surface_group::SurfaceGroup,
        }, textures::image::ImageTexture,
    },
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let material_ground: Arc<dyn Material> =
        Arc::new(Lambertian::new_from_color(Color::new(0.8, 0.8, 0.4)));

    let material_center: Arc<dyn Material> =
        Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.3, 0.6)));

    let material_left: Arc<dyn Material> =
        Arc::new(Dielectric::new(1.51));

    let material_right: Arc<dyn Material> =
        Arc::new(Metal::new(Color::new(0.8, 0.2, 0.8), 0.3));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Arc::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
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
        5.0,
        1.0,
    );

    camera.upd_pos(
        Some(90.0),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 0.0, -1.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(ImageTexture::new(open("textures/sky_1.jpg").expect("Failed to open skybox image, make sure you are in project root!").to_rgb8()));

    (world, camera)
}