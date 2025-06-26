pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

// use color::Color;
use crate::camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
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

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    cam.render(&world);
}
