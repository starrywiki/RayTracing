// sphere.rs
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::rtweekend;
use crate::vec3;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Arc<dyn Material + Send + Sync>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    /// 创建新球体
    pub fn new_static(
        center1: Point3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            center1,
            radius,
            mat: material,
            is_moving: false,
            center_vec: Vec3::new(0.0, 0.0, 0.0),
            bbox: Aabb::new_points(
                center1 - Vec3::new(radius, radius, radius),
                center1 + Vec3::new(radius, radius, radius),
            ),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_points(center1 - rvec, center1 + rvec);
        let box2 = Aabb::new_points(center2 - rvec, center2 + rvec);
        Self {
            center1,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: Aabb::new_boxes(&box1, &box2),
        }
    }

    fn center(&self, time: f64) -> Point3 {
        if self.is_moving {
            self.center1 + time * self.center_vec
        } else {
            self.center1
        }
    }

    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + rtweekend::PI;
        let u = phi / (2.0 * rtweekend::PI) as f64;
        let v = (theta / rtweekend::PI) as f64;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let cur_center = self.center(r.time());
        let oc = cur_center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // 寻找最近的且在有效区间内的根
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        // 填充命中记录
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - cur_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        (rec.u, rec.v) = Sphere::get_sphere_uv(outward_normal);
        rec.mat = Arc::clone(&self.mat);
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
