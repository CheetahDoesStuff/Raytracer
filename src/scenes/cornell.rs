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
    }, transform::Transform,
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

    let box1 = Arc::new(quad_box(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 330.0, 165.0),
        white.clone()
    ));
    let box1 = Arc::new(Transform::rotate_y(box1, 15.0));
    let box1 = Arc::new(Transform::translate(box1, 265.0, 0.0, 295.0));
    world.add(box1);
    
    let box2 = Arc::new(quad_box(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 165.0, 165.0),
        white.clone()
    ));
    let box2 = Arc::new(Transform::rotate_y(box2, -18.0));
    let box2 = Arc::new(Transform::translate(box2, 130.0, 0.0, 65.0));
    world.add(box2);

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