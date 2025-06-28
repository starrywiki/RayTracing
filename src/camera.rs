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
    pub aspect_ratio: f64,        // Ratio of image
    pub image_width: i32,         // Rendered image width
    pub image_height: i32,        // Rendered image height
    pub samples_per_pixel: usize, // 像素采样数
    // 视角参数
    pub max_depth: i32,   // 光线最大反弹深度
    pub vfov: f64,        // 垂直视野角度（degrees）
    pub lookfrom: Point3, // 相机位置
    pub lookat: Point3,   // 观察目标点
    pub vup: Vec3,        // 相机的"向上"方向向量
    // 景深效果
    pub defocus_angle: f64, // 失焦角度
    pub focus_dist: f64,    // 对焦距离

    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    // 相机坐标系基向量
    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3, // 失焦盘的水平半径
    defocus_disk_v: Vec3, // 失焦盘的垂直半径
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}
impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        let mut world = HittableList::default();

        // 相机到视口的距离（焦距）
        let focal_length = (self.lookfrom - self.lookat).length();

        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        //相机坐标系的u,v,w单位基向量
        self.center = self.lookfrom;
        self.w = vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
        self.v = vec3::cross(self.w, self.u);

        // 水平和垂直视口边缘上的向量
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 视口左上角坐标
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        // (0,0)像素的中心位置
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let defocus_radius =
            self.focus_dist * (rtweekend::degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
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
                    pixel_color += Camera::ray_color(&r, self.max_depth, world);
                }
                pixel_color
                    .write_color(&mut stdout.lock(), self.samples_per_pixel)
                    .unwrap();
            }
        }

        eprintln!("\nDone.");
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
    /// 在像素区域内随机采样（用于抗锯齿）
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rtweekend::random_double();
        let py = -0.5 + rtweekend::random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
    /// 在失焦盘上随机采样（用于景深效果）
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = {
            if self.defocus_angle <= 0.0 {
                self.center
            } else {
                self.defocus_disk_sample()
            }
        };
        let ray_direc = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direc)
    }
}
