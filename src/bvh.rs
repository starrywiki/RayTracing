// bvh.rs
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend;
use std::cmp::Ordering;
use std::sync::Arc;

/// BVH 节点结构体
pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>]) -> Self {
        let axis = rtweekend::random_int_range(0, 2); // 0:x, 1:y, 2:z

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = objects.len();
        let (left, right): (
            Arc<dyn Hittable + Send + Sync>,
            Arc<dyn Hittable + Send + Sync>,
        ) = match object_span {
            1 => {
                let obj = Arc::clone(&objects[0]);
                (Arc::clone(&obj), obj)
            }
            2 => {
                let (a, b) = if comparator(&objects[0], &objects[1]) {
                    (Arc::clone(&objects[0]), Arc::clone(&objects[1]))
                } else {
                    (Arc::clone(&objects[1]), Arc::clone(&objects[0]))
                };
                (a, b)
            }
            _ => {
                objects.sort_by(|a, b| {
                    if comparator(a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                let mid = object_span / 2;
                let left = Arc::new(BvhNode::new(&mut objects[..mid]));
                let right = Arc::new(BvhNode::new(&mut objects[mid..]));
                (left, right)
            }
        };

        let bbox = Aabb::new_boxes(left.bounding_box(), right.bounding_box());
        Self { left, right, bbox }
    }
}

fn box_compare(
    a: &Arc<dyn Hittable + Send + Sync>,
    b: &Arc<dyn Hittable + Send + Sync>,
    axis: usize,
) -> bool {
    a.bounding_box().axis_interval(axis).min < b.bounding_box().axis_interval(axis).min
}

fn box_x_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> bool {
    box_compare(a, b, 2)
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, *ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            &Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
