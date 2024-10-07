use crate::hittable::HitRecord;
use crate::hittable::Hittable;

use crate::color::write_color;
use crate::color::Color;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::util;
use crate::vec3::unit_vector;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use image::{ImageBuffer, Rgba};
use std::io::Write;

pub struct CameraParams {
    pub aspect_ratio: f64,
    pub img_width: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

pub struct Camera {
    pub aspect_ratio: f64,      // Aspect ratio of the image
    pub img_width: usize,       // Rendered image width
    pub img_height: usize,      // Rendered image height
    pub samples_per_pixel: u32, // Number of samples per pixel
    pub max_depth: u32,         // Maximum recursion depth
    pub vfov: f64,              // Vertical field of view in degrees
    pub lookfrom: Point3,       // Camera position
    pub lookat: Point3,         // Point the camera is looking at
    pub vup: Vec3,              // Up vector of the camera
    pub defocus_angle: f64,     // Defocus angle
    pub focus_dist: f64,        // Distance to focus plane

    pixel_samples_scale: f64, // Color scale factor for pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel (0,0)
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    u: Vec3,                  // Camera frame basis vector u
    v: Vec3,                  // Camera frame basis vector v
    w: Vec3,                  // Camera frame basis vector w
    defocus_disk_u: Vec3,     // Horizontal defocus disk radius
    defocus_disk_v: Vec3,     // Vertical defocus disk radius
}

impl Camera {
    pub fn new_default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            img_width: 100,
            img_height: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            pixel_samples_scale: 1.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_basic_params(&mut self, params: CameraParams) {
        self.aspect_ratio = params.aspect_ratio;
        self.img_width = params.img_width;
        self.samples_per_pixel = params.samples_per_pixel;
        self.max_depth = params.max_depth;
        self.vfov = params.vfov;
        self.lookfrom = params.lookfrom;
        self.lookat = params.lookat;
        self.vup = params.vup;
        self.defocus_angle = params.defocus_angle;
        self.focus_dist = params.focus_dist;
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        println!("\nP3");
        println!("{} {}", self.img_width, self.img_height);
        println!("255");

        let mut pixel_data = vec![0u8; self.img_width * self.img_height * 4]; // RGBA format

        for j in (0..self.img_height).rev() {
            eprint!("\rScanlines remaining: {}", j);
            std::io::stdout().flush().unwrap();
            for i in 0..self.img_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
                }
                write_color(
                    self.pixel_samples_scale * pixel_color,
                    &mut pixel_data,
                    self.img_width,
                    j,
                    i,
                );
            }
        }

        eprintln!("\rDone.");

        let img_buffer: ImageBuffer<Rgba<u8>, _> =
            ImageBuffer::from_raw(self.img_width as u32, self.img_height as u32, pixel_data)
                .unwrap();
        img_buffer.save("output.png").unwrap()
    }

    pub fn initialize(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio).round() as usize;
        if self.img_height < 1 {
            self.img_height = 1;
        }
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;
        let theta = util::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(self.vup.cross(&self.w));
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.img_width as f64;
        self.pixel_delta_v = viewport_v / self.img_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_dist * (util::degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn sample_square(&self) -> Vec3 {
        Vec3::new(
            util::random_double() - 0.5,
            util::random_double() - 0.5,
            0.0,
        )
    }

    pub fn sample_disk(&self, radius: f64) -> Vec3 {
        radius * self.random_in_unit_disk()
    }

    fn random_in_unit_disk(&self) -> Vec3 {
        loop {
            let p = Vec3::new(
                util::random_double() * 2.0 - 1.0,
                util::random_double() * 2.0 - 1.0,
                0.0,
            );
            if p.length_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn defocus_disk_sample(&self) -> Point3 {
        let p = self.random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    pub fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        if world.hit(r, &Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
