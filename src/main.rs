pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::sync::Arc;
use vec3::{Point3, Vec3};
fn main() {
    // camera
    let mut cam = Camera::default();
    cam.image_width = 400;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 50.0;

    let mut world = HittableList::default();
    let R = (rtweekend::PI / 4.0).cos();
    let mat_l = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_r = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    world.add(Arc::new(Sphere::new(Point3::new(-R, 0.0, -1.0), R, mat_l)));
    world.add(Arc::new(Sphere::new(Point3::new(R, 0.0, -1.0), R, mat_r)));
    // let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let mat_left = Arc::new(Dielectric::new(1.50));
    // let mat_bubble = Arc::new(Dielectric::new(1.0 / 1.50));
    // let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     mat_ground,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     mat_center,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     mat_left,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     mat_bubble,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     mat_right,
    // )));

    cam.render(&world);
}
