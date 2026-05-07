use f32;
use nalgebra::Vector3;
use raytracer::ray::color::{Color, write_col};
use raytracer::ray::ray::Ray;
use raytracer::surface::sphere::Sphere;
use raytracer::surface::surface::{HitRecord, Surface};
use raytracer::surface::surface_group::SurfaceGroup;
use std::io::{self, Write};
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn ray_color(ray: &Ray, world: &dyn Surface) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(ray, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vector3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let white = Color::new(0.5, 0.5, 1.0);
    let sky = Color::new(0.0, 0.0, 0.8);
    white * (1.0 - a) + sky * a
}

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);

    let mut world = SurfaceGroup::new();
    world.add(
        Arc::new(
            Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)
    ));
    world.add(
        Arc::new(
            Sphere::new(Vector3::new(0.8, 0.0, -1.0), 0.3)
    ));
    world.add(
        Arc::new(
            Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)
        )
    );

    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vector3::new(0.0f32, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width as f32, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    //  Math got so complicated i cant even understand this anymore :sob-prism:
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    println!("{}", header);

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        io::stderr().flush().unwrap();

        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_dir);

            let col = ray_color(&ray, &world);
            write_col(&col);
        }
    }

    eprint!("\rDone!                 \n");
    io::stderr().flush().unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    eprintln!("Took: {:?}", time)
}
