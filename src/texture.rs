//texture.rs
use crate::color::Color;
use crate::perlin::Perlin;
use crate::rtw_image::RtwImage;
use crate::rtweekend;
use crate::vec3::Point3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i32;
        let y_int = (self.inv_scale * p.y).floor() as i32;
        let z_int = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Option<Self> {
        {
            RtwImage::new(filename).map(|img| Self { image: img })
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.image.height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let i = (u * self.image.width as f64) as usize;
        let j = (v * self.image.height as f64) as usize;

        let i = i.min(self.image.width - 1);
        let j = j.min(self.image.height - 1);

        let pixel = self.image.pixel_data(i as i32, j as i32);

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(siz: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: siz,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.turb(p, 7)
    }
}
