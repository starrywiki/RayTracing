//constant_medium.rs
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::rtweekend::{random_double, INFINITY};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Point3, Vec3};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Send + Sync>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material + Send + Sync>,
}

impl ConstantMedium {
    // 构造一个使用纹理散射相函数的常密度体
    pub fn new_with_texture(
        boundary: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        texture: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_texture(texture)),
        }
    }

    // 构造一个使用纯色的常密度体
    pub fn new_with_color(
        boundary: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        color: Vec3,
    ) -> Self {
        let texture = Arc::new(SolidColor::new(color));
        Self::new_with_texture(boundary, density, texture)
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        // 第一次命中 boundary
        if !self.boundary.hit(r, &Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        // 第二次命中 boundary（出界）
        if !self
            .boundary
            .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        let mut t1 = rec1.t.max(ray_t.min);
        let mut t2 = rec2.t.min(ray_t.max);

        if t1 >= t2 {
            return false;
        }

        if t1 < 0.0 {
            t1 = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = t1 + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        // 体积散射不关心表面法线与朝向
        rec.normal = Vec3::new(1.0, 0.0, 0.0); // 任意法线
        rec.front_face = true;
        rec.mat = Arc::clone(&self.phase_function);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
