//camera.rs
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray;
use crate::ray::Ray;
use crate::rtweekend;
use crate::sphere::Sphere;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use crossbeam::thread;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Condvar;
use std::sync::{Arc, Mutex};

// 多线程参数
const HEIGHT_PARTITION: usize = 20;
const WIDTH_PARTITION: usize = 20;
const THREAD_LIMIT: usize = 16;

#[derive(Clone)]
pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image
    pub image_width: i32,         // Rendered image width
    pub image_height: i32,        // Rendered image height
    pub samples_per_pixel: usize, // 像素采样数
    // 视角参数
    pub max_depth: i32, // 光线最大反弹深度
    pub background: Color,
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
            background: Color::default(),
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

        let world = HittableList::default();

        // 相机到视口的距离（焦距）
        // let focal_length = (self.lookfrom - self.lookat).length();

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
        let progress = Arc::new(AtomicUsize::new(0));
        let total_lines = self.image_height as usize;

        // 输出 PPM 头部
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let stdout = stdout();

        // 初始化图像缓冲区
        let mut image: Vec<Vec<Color>> =
            vec![vec![Color::default(); self.image_width as usize]; self.image_height as usize];
        let image_mutex = Arc::new(Mutex::new(&mut image));

        // 包装 Camera 和 world
        let camera = Arc::new(self.clone());
        let world = Arc::new(world as &(dyn Hittable + Send + Sync));

        // 线程计数和条件变量
        let thread_count = Arc::new(AtomicUsize::new(0));
        let thread_number_controller = Arc::new(Condvar::new());

        // 计算分块大小
        let chunk_height = (self.image_height as usize + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;
        let chunk_width = (self.image_width as usize + WIDTH_PARTITION - 1) / WIDTH_PARTITION;

        thread::scope(|thd_spawner| {
            for j in 0..HEIGHT_PARTITION {
                for i in 0..WIDTH_PARTITION {
                    // 等待线程数量低于限制
                    let lock_for_condv = Mutex::new(false);
                    while thread_count.load(Ordering::SeqCst) >= THREAD_LIMIT {
                        thread_number_controller
                            .wait(lock_for_condv.lock().unwrap())
                            .unwrap();
                    }

                    // 克隆 Arc
                    let camera = Arc::clone(&camera);
                    let world = Arc::clone(&world);
                    let image_mutex = Arc::clone(&image_mutex);
                    let thread_count = Arc::clone(&thread_count);
                    let thread_number_controller = Arc::clone(&thread_number_controller);

                    // 增加线程计数
                    thread_count.fetch_add(1, Ordering::SeqCst);
                    let progress = Arc::clone(&progress);
                    // 启动子线程
                    thd_spawner.spawn(move |_| {
                        camera.render_sub(
                            *world,
                            &image_mutex,
                            i * chunk_width,
                            (i + 1) * chunk_width,
                            j * chunk_height,
                            (j + 1) * chunk_height,
                            total_lines,
                            &progress,
                        );
                        // 线程结束，减少计数并通知
                        thread_count.fetch_sub(1, Ordering::SeqCst);
                        thread_number_controller.notify_one();
                    });
                }
            }
        })
        .unwrap();

        // 输出图像
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                image[j as usize][i as usize]
                    .write_color(&mut stdout.lock(), self.samples_per_pixel)
                    .unwrap();
            }
        }

        eprintln!("\nDone.");
    }

    fn render_sub(
        &self,
        world: &dyn Hittable,
        image_mutex: &Arc<Mutex<&mut Vec<Vec<Color>>>>,
        x_min: usize,
        x_max: usize,
        y_min: usize,
        y_max: usize,
        total_lines: usize,
        progress: &Arc<AtomicUsize>,
        // samples_per_pixel: usize,
    ) {
        // 限制边界
        let x_max = x_max.min(self.image_width as usize);
        let y_max = y_max.min(self.image_height as usize);

        // 临时缓冲区
        let mut buffer = vec![vec![Color::default(); x_max - x_min]; y_max - y_min];

        // 渲染子区域
        for j in y_min..y_max {
            for i in x_min..x_max {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as i32, j as i32);
                    pixel_color += Self::ray_color(self, &r, self.max_depth, world);
                }
                buffer[j - y_min][i - x_min] = pixel_color;
            }
        }
        // 实时进度更新（每渲染完一行调用）
        let done = progress.fetch_add(1, Ordering::SeqCst) + 1;
        eprint!(
            "\rScanlines remaining: {}",
            total_lines.saturating_sub(done)
        );
        std::io::stderr().flush().unwrap();
        // 将缓冲区写入图像
        let mut image = image_mutex.lock().unwrap();
        for j in y_min..y_max {
            for i in x_min..x_max {
                image[j][i] = buffer[j - y_min][i - x_min];
            }
        }
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if !world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }
        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);
        color_from_emission + color_from_scatter
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
        let ray_tm = rtweekend::random_double();
        Ray::new(ray_origin, ray_direc, ray_tm)
    }
}
