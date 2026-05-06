use f32;
use std::io::{self, Write};
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use nalgebra::Vector3;
use raytracer::ray::color::{Color, write_col};
use raytracer::ray::ray::Ray;

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = center - ray.origin();

    let a = ray.direction().norm_squared();
    let h = ray.direction().dot(&oc);
    let c = oc.norm_squared() - radius * radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let center = Vector3::new(0.0, 0.0, -1.0);
        let n = (ray.at(t) - center).normalize();

        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
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

    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vector3::new(0.0f32, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width as f32, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    //  Math got so complicated i cant even understand this anymore :sob-prism:
    let viewport_upper_left = camera_center 
                                - Vector3::new(0.0, 0.0, focal_length)
                                - viewport_u / 2.0
                                - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); 

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    println!("{}", header);

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        io::stderr().flush().unwrap();

        for x in 0..image_width {
            let pixel_center =
                                pixel00_loc
                                + (x as f32 * pixel_delta_u)
                                + (y as f32 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_dir);

            let col = ray_color(&r);
            write_col(&col);
        }
    }

    eprint!("\rDone!                 \n");
    io::stderr().flush().unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    eprintln!("Took: {:?}", time)
}