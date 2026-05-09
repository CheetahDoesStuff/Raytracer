use crate::{
    interval::Interval,
    ray::{
        color::{Color, write_col_string},
        ray::Ray,
    },
    surface::surface::{HitRecord, Surface},
    utils::{INFINITY, random_f32},
};
use nalgebra::Vector3;
use std::io::{self, Write};
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

        let viewport_upper_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
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
            pixel_delta_v: pixel_delta_v,
        }
    }

    pub fn render(self: &Self, world: &dyn Surface) {
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        println!("{}", header);

        let remaining = Arc::new(AtomicI32::new(self.image_height as i32));
        let total_height = self.image_height as i32;
        let remaining_clone = Arc::clone(&remaining);
        thread::spawn(move || {
            loop {
                let r = remaining_clone.load(Ordering::Relaxed);
                eprint!("\rRendering scanlines: {} / {}", total_height as i32 - r, total_height);
                io::stderr().flush().unwrap();
                if r == 0 { break; }
                thread::sleep(Duration::from_millis(50));
            }
        });

        let rows: Vec<String> = (0..self.image_height as i32)
                .into_par_iter()
                .map(|y| {
            let remaining = Arc::clone(&remaining);
            remaining.fetch_sub(1, Ordering::Relaxed);

            let mut row = String::new();

            for x in 0..self.image_width as i32 {
                let mut col = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel as i32 {
                    let ray = self.get_ray(x, y);
                    col += self.ray_color(&ray, &world, 50)
                }
                row.push_str(&write_col_string(&(col * self.pixel_sample_scale)));
            }
            row
        }).collect();

        eprintln!();
        for (i, row) in rows.into_iter().enumerate() {
            eprint!("\rWriting scanlines: {} / {}", i + 1, total_height);
            io::stderr().flush().unwrap();
            print!("{}", row); 
        }

        eprint!("\nDone!\n");
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
        Vector3::new(
            random_f32(None, None) - 0.5,
            random_f32(None, None) - 0.5,
            0.0,
        )
    }

    fn ray_color(&self, ray: &Ray, world: &&dyn Surface, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::default();

            if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation.component_mul(&self.ray_color(&scattered, world, depth - 1));
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        let sky = Color::new(0.5, 0.7, 1.0);

        white * (1.0 - a) + sky * a
    }
}
