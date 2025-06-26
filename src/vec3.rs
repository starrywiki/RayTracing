use crate::rtweekend;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};
/// 3D 向量类型，也可用于表示点和颜色
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// 类型别名（类似 C++ 的 using point3 = vec3）
pub type Point3 = Vec3;

impl Vec3 {
    /// 创建零向量
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// 构造函数
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// 向量长度平方
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// 向量长度
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// 单位向量
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

// 运算符重载 ============================================

// 负号 (-v)
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// 加法 (v1 + v2)
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// 减法 (v1 - v2)
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// 点乘
impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// 标量乘法 (t * v 和 v * t)
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

// self 为标量
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// 标量除法 (v / t)
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

// +=
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

// *=
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self::new(self.x * t, self.y * t, self.z * t);
    }
}

//  /=
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

// 索引访问 v[i]（只读）
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

// 索引访问 v[i]（可变）
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

// 格式化输出
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// 点积
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

// 单位向量
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// 叉乘
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn random() -> Vec3 {
    Vec3::new(
        rtweekend::random_double(),
        rtweekend::random_double(),
        rtweekend::random_double(),
    )
}

pub fn random_double_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        rtweekend::random_double_range(min, max),
        rtweekend::random_double_range(min, max),
        rtweekend::random_double_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_double_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
