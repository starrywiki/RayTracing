// interval.rs
use crate::rtweekend;

#[derive(Default, Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn union(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    /// Returns the size of the interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if the interval contains a value (inclusive)
    pub fn contains(&self, x: f64) -> bool {
        x <= self.max && self.min <= x
    }

    /// Checks if the interval surrounds a value (exclusive)
    pub fn surrounds(&self, x: f64) -> bool {
        x < self.max && self.min < x
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    /// Empty interval constant
    pub const EMPTY: Interval = Interval {
        min: rtweekend::INFINITY,
        max: -rtweekend::INFINITY,
    };

    /// Universe interval constant (all real numbers)
    pub const UNIVERSE: Interval = Interval {
        min: -rtweekend::INFINITY,
        max: rtweekend::INFINITY,
    };

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}
