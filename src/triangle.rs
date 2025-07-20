// triangle.rs
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{cross, dot, Point3, Vec3};
use std::sync::Arc;

pub struct Triangle {
    p0: Point3,
    // 为了加速计算，我们预先存储边和法线
    edge1: Vec3,
    edge2: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
    // 我们可以预计算法线，但如果模型带有法线数据，则情况更复杂
    // 为简单起见，我们先自己计算
    normal: Vec3,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Point3, p2: Point3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let edge1 = p1 - p0;
        let edge2 = p2 - p0;
        let normal = cross(edge1, edge2).unit_vector();
        let raw_bbox = Aabb::new_boxes(&Aabb::new_points(p0, p1), &Aabb::new_points(p2, p2));
        let final_bbox = raw_bbox.pad();
        Self {
            p0,
            edge1,
            edge2,
            mat,
            bbox: final_bbox,
            normal,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // 实现 Möller–Trumbore 算法
        let pvec = cross(r.direction(), self.edge2);
        let det = dot(self.edge1, pvec);

        // 如果 det 接近 0，说明射线平行于三角形平面
        if det.abs() < 1e-8 {
            return false;
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin() - self.p0;

        // 计算重心坐标 u
        let u = dot(tvec, pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let qvec = cross(tvec, self.edge1);

        // 计算重心坐标 v
        let v = dot(r.direction(), qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        // 计算交点 t
        let t = dot(self.edge2, qvec) * inv_det;
        if !ray_t.contains(t) {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.mat = Arc::clone(&self.mat);
        // 这里我们暂时使用几何法线，后续可以支持平滑着色的顶点法线
        rec.set_face_normal(r, self.normal);

        // 你也可以选择在这里计算并填充纹理坐标 u, v
        rec.u = u;
        rec.v = v;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
