use nalgebra::Vector3;
use std::sync::Arc;

use crate::{
    camera::Camera,
    ray::color::Color,
    surface::{
        materials::{
            diffuse_light::DiffuseLight, lambertian::Lambertian
        },
        surfaces::{
            quad::{Quad, quad_box},
            surface_group::SurfaceGroup,
        }, textures::solid_color::SolidColorTexture,
    },
};

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let red = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColorTexture::new(Color::new(15.0, 15.0, 15.0)))));

    world.add(Arc::new(Quad::new(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white.clone()
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white.clone()
    )));

    world.add(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white.clone()
    )));

    world.add(Arc::new(quad_box(Vector3::new(130.0, 0.0, 65.0), Vector3::new(295.0, 165.0, 230.0), white.clone())));
    world.add(Arc::new(quad_box(Vector3::new(265.0, 0.0, 295.0), Vector3::new(430.0, 330.0, 460.0), white.clone())));

    #[cfg(feature = "denoise")]
    let samples = 100;
    #[cfg(not(feature = "denoise"))]
    let samples = 100;

    let mut camera = Camera::new(
        1.0,
        600,
        samples,
        90.0,
        0.0,
        1.0,
    );

    camera.upd_pos(
        Some(40.0),
        Some(Vector3::new(278.0, 278.0, -800.0)),
        Some(Vector3::new(278.0, 278.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    camera.set_skybox(
        SolidColorTexture::new(Color::new(0.0, 0.0, 0.0))
    );

    (world, camera)
}