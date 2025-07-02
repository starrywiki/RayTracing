//aabb.rs
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            x: Interval::new(f64::INFINITY, f64::NEG_INFINITY),
            y: Interval::new(f64::INFINITY, f64::NEG_INFINITY),
            z: Interval::new(f64::INFINITY, f64::NEG_INFINITY),
        }
    }
    pub fn new_boxes(box1: Aabb, box2: Aabb) -> Self {
        Self {
            x: Interval::union(box1.x, box2.x),
            y: Interval::union(box1.y, box2.y),
            z: Interval::union(box1.z, box2.z),
        }
    }
    /// 使用 3 个方向区间构造 AABB
    pub fn new_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// 使用两个点构造 AABB，不要求顺序
    pub fn new_points(a: Point3, b: Point3) -> Self {
        Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }
    }

    /// 获取第 n 个方向的区间（0:x, 1:y, 2:z）
    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    /// 判断 ray 是否与 AABB 相交（ray_t 为合法区间）
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let interval = self.axis_interval(axis);
            let inv_d = 1.0 / ray_dir[axis];

            let mut t0 = (interval.min - ray_orig[axis]) * inv_d;
            let mut t1 = (interval.max - ray_orig[axis]) * inv_d;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            ray_t.min = ray_t.min.max(t0);
            ray_t.max = ray_t.max.min(t1);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
