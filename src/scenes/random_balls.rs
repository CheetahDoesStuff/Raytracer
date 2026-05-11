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
        },
    },
    utils::random_f32,
};

fn random_vec3(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        random_f32(Some(min), Some(max)),
        random_f32(Some(min), Some(max)),
        random_f32(Some(min), Some(max)),
    )
}

pub fn scene() -> (SurfaceGroup, Camera) {
    let mut world = SurfaceGroup::new();

    let ground_material: Arc<dyn Material> =
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32(None, None);

            let center = Vector3::new(
                a as f32 + 0.9 * random_f32(None, None),
                0.2,
                b as f32 + 0.9 * random_f32(None, None),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = random_vec3(0.0, 1.0)
                        .component_mul(&random_vec3(0.0, 1.0));

                    sphere_material = Arc::new(Lambertian::new(albedo));

                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3(0.5, 1.0);
                    let fuzz = random_f32(Some(0.0), Some(0.5));

                    sphere_material = Arc::new(Metal::new(albedo, fuzz));

                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));

                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1: Arc<dyn Material> =
        Arc::new(Dielectric::new(1.5));

    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2: Arc<dyn Material> =
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));

    world.add(Arc::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3: Arc<dyn Material> =
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        20.0,
        0.6,
        10.0,
    );

    camera.upd_pos(
        Some(20.0),
        Some(Vector3::new(13.0, 2.0, 3.0)),
        Some(Vector3::new(0.0, 0.0, 0.0)),
        Some(Vector3::new(0.0, 1.0, 0.0)),
    );

    (world, camera)
}