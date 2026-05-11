use f32;
use nalgebra::Vector3;
use raytracer::camera::Camera;
use raytracer::ray::color::Color;
use raytracer::surface::material::Material;
use raytracer::surface::materials::dielectric::Dielectric;
use raytracer::surface::materials::lambertian::Lambertian;
use raytracer::surface::materials::metal::Metal;
use raytracer::surface::surfaces::sphere::Sphere;
use raytracer::surface::surfaces::surface_group::SurfaceGroup;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(aspect_ratio, image_width, 100, 90.0);
    let mut world = SurfaceGroup::new();
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.4))); // Lambertian::new(Color::new(0.8, 0.8, 0.4))
    let material_center: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 0.6)));
    let material_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.51)); // Metal::new(Color::new(0.8, 0.8, 0.8), 0.8) Dielectric::new(1.5)
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.2, 0.8), 0.3));

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

    camera.render(&world);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    eprintln!("Took: {:?}", time)
}
