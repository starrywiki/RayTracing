//camera.rs
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend;

use crate::sphere::Sphere;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: i32,  // Rendered image width in pixel count
    pub image_height: i32, // Rendered image height
    pub samples_per_pixel: usize,
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            samples_per_pixel: 10,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}
impl Camera {
    fn initialize(&mut self) {
        // 计算图像高度，并确保至少为1。
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        let mut world = HittableList::default();
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        // 视口
        let focal_length = 1.0; // 相机到视口的距离（即“焦距”）
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point3::zero();

        // 计算水平和垂直视口边缘上的向量
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // 计算从像素到像素的水平和垂直增量向量
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 视口的左上角坐标
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        // 第一个像素（(0,0)像素）的中心位置
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let stdout = std::io::stdout();

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world);
                }
                pixel_color
                    .write_color(&mut stdout.lock(), self.samples_per_pixel)
                    .unwrap();
            }
        }

        eprintln!("\nDone.");
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, &Interval::new(0.0, rtweekend::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rtweekend::random_double();
        let py = -0.5 + rtweekend::random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direc = pixel_sample - self.center;
        Ray::new(self.center, ray_direc)
    }
}
