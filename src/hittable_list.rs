//hittable_list.rs
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::sync::Arc;
/// 可击中对象列表
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>) -> Self {
        Self {
            objects: vec![object],
            bbox: Aabb::empty(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // 添加可击中对象
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::new_boxes(self.bbox, object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
