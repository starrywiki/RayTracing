use crate::aabb;
use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::rtweekend;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
// 存储射线命中物体时的记录信息
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,    // 命中点坐标
    pub normal: Vec3, // 命中点法线（单位向量）
    pub t: f64,       // 射线参数 t
    pub front_face: bool,
    pub mat: Arc<dyn Material + Send + Sync>,
    pub u: f64,
    pub v: f64,
}

// 可命中物体的统一接口
pub trait Hittable: Send + Sync {
    // 判断射线是否命中物体，若命中则填充 HitRecord
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // assume outward_normal has unit length
        // 确保法线始终指向射线的入射方向的反方向
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: Arc::new(Lambertian::default()), // 或某个默认材质
            t: 0.0,
            front_face: false,
            u: 0.0,
            v: 0.0,
        }
    }
}

pub struct Translate {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let the_bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox: the_bbox,
        }
    }
    pub fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // 将光线反向偏移
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&moved_r, ray_t, rec) {
            return false;
        }

        // 将命中点偏移回来
        rec.p += self.offset;
        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle_deg: f64) -> Self {
        let radians = angle_deg.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // 计算原始包围盒
        let orig_bbox = object.bounding_box();

        let mut min = Point3::new(
            rtweekend::INFINITY,
            rtweekend::INFINITY,
            rtweekend::INFINITY,
        );
        let mut max = Point3::new(
            -rtweekend::INFINITY,
            -rtweekend::INFINITY,
            -rtweekend::INFINITY,
        );

        if let b = orig_bbox {
            // 遍历包围盒 8 个角点变换后的坐标，重新生成旋转后的 bbox
            for i in 0..2 {
                let x = if i == 0 { b.x.min } else { b.x.max };
                for j in 0..2 {
                    let y = if j == 0 { b.y.min } else { b.y.max };
                    for k in 0..2 {
                        let z = if k == 0 { b.z.min } else { b.z.max };

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            Self {
                object,
                sin_theta,
                cos_theta,
                bbox: Aabb::new_points(min, max),
            }
        } else {
            Self {
                object,
                sin_theta,
                cos_theta,
                bbox: Aabb::default(),
            }
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // Ray: 世界空间 -> 物体空间
        let origin = Point3::new(
            self.cos_theta * r.origin().x - self.sin_theta * r.origin().z,
            r.origin().y,
            self.cos_theta * r.origin().z + self.sin_theta * r.origin().x,
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction().x - self.sin_theta * r.direction().z,
            r.direction().y,
            self.cos_theta * r.direction().z + self.sin_theta * r.direction().x,
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // 命中点和法线旋转回世界空间
        let p = Point3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );

        let normal = Vec3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
