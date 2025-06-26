// ray.rs
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    //  origin: 光线起点  direction: 光线方向
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // 计算光线在参数 t 处的位置
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

// 默认实现
impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Vec3::zero(),
            dir: Vec3::zero(),
        }
    }
}
