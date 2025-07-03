//perlin.rs
use crate::rtweekend;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
#[derive(Clone)]
pub struct Perlin {
    randvec: [Vec3; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            randvec[i] = vec3::random_unit_vector();
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.randvec[idx];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            let u_weight = i as f64 * u + (1.0 - i as f64) * (1.0 - u);
            for j in 0..2 {
                let v_weight = j as f64 * v + (1.0 - j as f64) * (1.0 - v);
                for k in 0..2 {
                    let w_weight = k as f64 * w + (1.0 - k as f64) * (1.0 - w);
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += u_weight * v_weight * w_weight * vec3::dot(c[i][j][k], weight_v);
                }
            }
        }

        accum
    }

    fn generate_perm() -> [usize; Self::POINT_COUNT] {
        let mut p = [0usize; Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; Self::POINT_COUNT]) {
        for i in (1..Self::POINT_COUNT).rev() {
            let target = rtweekend::random_int_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum: f64 = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
