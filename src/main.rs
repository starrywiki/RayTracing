pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtw_image;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod vec3;

use crate::quad::Quad;
use bvh::BvhNode;
use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use perlin::Perlin;
use ray::Ray;
use rtw_image::RtwImage;
use sphere::Sphere;
use std::sync::Arc;
use texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
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
    cam.background = Color::new(0.70, 0.80, 1.00);

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
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    cam.render(&world);
}

fn earth() {
    let earth_texture = ImageTexture::new("earthmap.jpg").expect("failed");
    let earth_texture: Arc<dyn Texture> = Arc::new(earth_texture);
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Arc::new(Sphere::new_static(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    let mut cam = Camera::default();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    cam.render(&HittableList::new(globe));
}

fn perlin_spheres() {
    let mut world = HittableList::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_from_texture(pertext)),
    )));

    let mut cam = Camera::default();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::default();
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::default();
    cam.image_width = 1200;
    cam.aspect_ratio = 1.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_from_texture(pertext)),
    )));
    let difflight = Arc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));
    let mut cam = Camera::default();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    world.add(Arc::new(quad::boxx(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        white.clone(),
    )));
    world.add(Arc::new(quad::boxx(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        white.clone(),
    )));
    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    let scene_id = 7;

    match scene_id {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => panic!("Unknown scene_id: {scene_id}"),
    }
}
