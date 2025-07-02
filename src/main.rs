pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod vec3;

use bvh::BvhNode;
use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::sync::Arc;
use texture::CheckerTexture;
use vec3::{Point3, Vec3};

fn bouncing_spheres() {
    let mut world = HittableList::default();
    // let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new_static(
    //     Point3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     mat_ground,
    // )));
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.5, 0.5, 0.5),
    ));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(checker)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let cur_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if cur_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::random() * vec3::random();
                    let mat_sphere = Arc::new(Lambertian::new(albedo));
                    let center2 =
                        center + Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center, center2, 0.2, mat_sphere,
                    )));
                } else if cur_mat < 0.95 {
                    //metal
                    let albedo = vec3::random_double_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    let mat_sphere = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, mat_sphere)));
                } else {
                    //glass
                    let mat_sphere = Arc::new(Dielectric::new(1.50));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, mat_sphere)));
                };
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.50));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
    )));
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(BvhNode::new(&mut world.objects));

    // camera
    let mut cam = Camera::default();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&*world);
}
fn checkered_spheres() {
    let mut world = HittableList::default();
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_from_texture(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_from_texture(checker)),
    )));

    let mut cam = Camera::default();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    cam.render(&world);
}
fn main() {
    let scene_id = 2;

    match scene_id {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        _ => panic!("Unknown scene_id: {scene_id}"),
    }
}
