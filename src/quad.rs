//quad.rs
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
pub struct Quad {
    q: Point3,                            // 起始角点
    u: Vec3,                              // 边向量 u
    v: Vec3,                              // 边向量 v
    mat: Arc<dyn Material + Send + Sync>, // 材质
    bbox: Aabb,                           // 包围盒
    normal: Vec3,
    D: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let n = vec3::cross(u, v);
        let normal = vec3::unit_vector(n);
        let D = vec3::dot(normal, q);
        let w = n / vec3::dot(n, n);
        let mut quad = Self {
            q,
            u,
            v,
            mat,
            bbox: Aabb::default(),
            normal,
            D,
            w,
        };
        quad.set_bounding_box(); // 初始化时设置包围盒
        quad
    }

    fn set_bounding_box(&mut self) {
        // 构造两个对角线包围盒，再取并集
        let bbox1 = Aabb::new_points(self.q, self.q + self.u + self.v);
        let bbox2 = Aabb::new_points(self.q + self.u, self.q + self.v);
        self.bbox = Aabb::new_boxes(&bbox1, &bbox2);
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        // 若 a 或 b 超出 [0,1] 范围，说明不在矩形内部
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // 计算射线方向与平面法线的点积
        let denom = vec3::dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.D - vec3::dot(self.normal, r.origin())) / denom;

        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = vec3::dot(self.w, vec3::cross(planar_hitpt_vector, self.v));
        let beta = vec3::dot(self.w, vec3::cross(self.u, planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Arc::clone(&self.mat);
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
pub fn boxx(a: Point3, b: Point3, mat: Arc<dyn Material + Send + Sync>) -> HittableList {
    let mut sides = HittableList::default();

    // 计算最小、最大点
    let min_pt = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max_pt = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max_pt.x - min_pt.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max_pt.y - min_pt.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max_pt.z - min_pt.z);

    // front
    sides.add(Arc::new(Quad::new(
        Point3::new(min_pt.x, min_pt.y, max_pt.z),
        dx,
        dy,
        Arc::clone(&mat),
    )));
    // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max_pt.x, min_pt.y, max_pt.z),
        -dz,
        dy,
        Arc::clone(&mat),
    )));
    // back
    sides.add(Arc::new(Quad::new(
        Point3::new(max_pt.x, min_pt.y, min_pt.z),
        -dx,
        dy,
        Arc::clone(&mat),
    )));
    // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min_pt.x, min_pt.y, min_pt.z),
        dz,
        dy,
        Arc::clone(&mat),
    )));
    // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min_pt.x, max_pt.y, max_pt.z),
        dx,
        -dz,
        Arc::clone(&mat),
    )));
    // bottom
    sides.add(Arc::new(Quad::new(
        Point3::new(min_pt.x, min_pt.y, min_pt.z),
        dx,
        dz,
        Arc::clone(&mat),
    )));

    sides
}
