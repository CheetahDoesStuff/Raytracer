use std::io::{self, Write};
use nalgebra::Vector3;
use crate::{interval::Interval, ray::{color::{Color, write_col}, ray::Ray}, surface::surface::{HitRecord, Surface}, utils::{INFINITY, random_f32, random_on_hemisphere}};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: f32,
    pub samples_per_pixel: f32,

    image_height: f32,
    center: Vector3<f32>,
    pixel00_loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    pixel_sample_scale: f32,

}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32) -> Self {
        let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let camera_center = Vector3::new(0.0f32, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width as f32, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio: aspect_ratio, 
            image_width: image_width as f32,
            samples_per_pixel: samples_per_pixel as f32,

            image_height: image_width as f32 / aspect_ratio,
            center: Vector3::new(0.0, 0.0, 0.0),
            pixel00_loc: pixel00_loc,
            pixel_sample_scale: 1.0 / samples_per_pixel as f32,

            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v
        }
    }

    pub fn render(self: &Self, world: &dyn Surface) {
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        println!("{}", header);
    
        for y in 0..self.image_height as i32 {
            eprint!("\rScanlines remaining: {} ", self.image_height as i32 - y);
            io::stderr().flush().unwrap();

            for x in 0..self.image_width as i32 {
                let mut col = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel as i32 {
                    let ray = self.get_ray(x, y);
                    col += self.ray_color(&ray, &world)
                }
                write_col(&(&col * self.pixel_sample_scale));
            }
        }

        eprint!("\rDone!                 \n");
        io::stderr().flush().unwrap();
    }

    fn get_ray(self: &Self, x: i32, y: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
                            + ((x as f32 + offset.x) * self.pixel_delta_u)
                            + ((y as f32 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;
        
        Ray::new(ray_origin, ray_dir)
    }

    fn sample_square(self: &Self) -> Vector3<f32> {
        Vector3::new(random_f32(None, None) - 0.5, random_f32(None, None) - 0.5, 0.0)
    }

    fn ray_color(self: &Self, ray: &Ray, world: &&dyn Surface) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(ray, Interval::new(0.001, INFINITY), &mut rec) {
            let direction = random_on_hemisphere(rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), world)
        }

        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        let sky = Color::new(0.6, 0.6, 0.8);
        white * (1.0 - a) + sky * a
    }
}