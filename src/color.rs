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
        write!(
            out,
            "{} {} {}\n",
            (256.0 * rtweekend::INTENSITY.clamp(r)) as i32,
            (256.0 * rtweekend::INTENSITY.clamp(g)) as i32,
            (256.0 * rtweekend::INTENSITY.clamp(b)) as i32
        )
    }
}
// samples_per_pixel: usize,
