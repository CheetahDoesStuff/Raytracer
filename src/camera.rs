use crate::{
    interval::Interval,
    ray::{
        color::{Color, write_col_string},
        ray::Ray,
    },
    surface::surface::{HitRecord, Surface},
    utils::{INFINITY, degrees_to_radians, random_f32},
};
use nalgebra::Vector3;
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: f32,
    pub samples_per_pixel: f32,
    pub fov: f32,

    pub lookfrom: Vector3<f32>,
    pub lookat: Vector3<f32>,
    pub vup: Vector3<f32>,

    image_height: f32,
    center: Vector3<f32>,
    pixel00_loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    pixel_sample_scale: f32
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width / self.aspect_ratio).max(1.0);

        let focal_length = 1.0;

        let theta = degrees_to_radians(self.fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width / self.image_height);

        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(&w).normalize();
        let v = w.cross(&u);

        self.center = self.lookfrom;

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        let viewport_upper_left =
            self.center - focal_length * w - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel;
    }

    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32, fov: f32) -> Self {
        let mut cam = Camera {
            aspect_ratio,
            image_width: image_width as f32,
            samples_per_pixel: samples_per_pixel as f32,
            fov,

            lookfrom: Vector3::new(0.0, 0.0, 0.0),
            lookat: Vector3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),

            image_height: 0.0,
            center: Vector3::new(0.0, 0.0, 0.0),

            pixel00_loc: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),

            pixel_sample_scale: 0.0,
        };

        cam.initialize();
        cam
    }

    pub fn upd_pos(
        &mut self,
        fov: Option<f32>,
        lookfrom: Option<Vector3<f32>>,
        lookat: Option<Vector3<f32>>,
        vup: Option<Vector3<f32>>,
    ) {
        if let Some(f) = fov {
            self.fov = f;
        }
        if let Some(lf) = lookfrom {
            self.lookfrom = lf;
        }
        if let Some(la) = lookat {
            self.lookat = la;
        }
        if let Some(v) = vup {
            self.vup = v;
        }
        self.initialize();
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
                eprint!(
                    "\rRendering scanlines: {} / {}",
                    total_height as i32 - r,
                    total_height
                );
                io::stderr().flush().unwrap();
                if r == 0 {
                    break;
                }
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
            })
            .collect();

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
