use crate::rtweekend;
use crate::vec3::Vec3;
use std::io::Write;
pub type Color = Vec3;

impl Color {
    pub fn write_color(
        &self,
        out: &mut dyn Write,
        samples_per_pixel: usize,
    ) -> std::io::Result<()> {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = scale * self.x;
        let g = scale * self.y;
        let b = scale * self.z;

        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);
        write!(
            out,
            "{} {} {}\n",
            (256.0 * rtweekend::INTENSITY.clamp(r)) as i32,
            (256.0 * rtweekend::INTENSITY.clamp(g)) as i32,
            (256.0 * rtweekend::INTENSITY.clamp(b)) as i32
        )
    }
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    } else {
        return 0.0;
    }
}
