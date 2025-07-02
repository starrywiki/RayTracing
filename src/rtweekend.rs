use crate::interval::Interval;
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}
