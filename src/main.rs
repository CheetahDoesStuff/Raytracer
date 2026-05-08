use f32;
use nalgebra::Vector3;
use raytracer::surface::sphere::Sphere;
use raytracer::surface::surface_group::SurfaceGroup;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use raytracer::camera::Camera;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(aspect_ratio, image_width, 100);

    let mut world = SurfaceGroup::new();
    world.add(
        Arc::new(
            Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)
    ));
    world.add(
        Arc::new(
            Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)
        )
    );

    camera.render(&world);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    eprintln!("Took: {:?}", time)
}
