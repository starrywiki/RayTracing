// material.rs
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend;
use crate::texture::{SolidColor, Texture};
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
unsafe impl Send for Lambertian {}
unsafe impl Sync for Lambertian {}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::default()
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            tex: Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0))), // 默认黑色
        }
    }
}
// Lambertian 漫反射材质
#[derive(Clone)]
pub struct Lambertian {
    pub tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(c: Color) -> Self {
        Self {
            tex: Arc::new(crate::texture::SolidColor::new(c)),
        }
    }

    pub fn new_from_texture(t: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex: t }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, //光线衰减
        scattered: &mut Ray,     //散射后的光线
    ) -> bool {
        let mut scatter_direct = rec.normal + vec3::random_unit_vector();
        if scatter_direct.near_zero() {
            scatter_direct = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direct, r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}
// Metal  （镜面反射）
pub struct Metal {
    pub albedo: Arc<dyn Texture + Send + Sync>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
            fuzz: f.min(1.0),
        }
    }
    pub fn new_from_texture(texture: Arc<dyn Texture + Send + Sync>, f: f64) -> Self {
        Self {
            albedo: texture,
            fuzz: f.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, //衰减
        scattered: &mut Ray,     //散射光线
    ) -> bool {
        let mut reflected = vec3::reflect(r_in.direction(), rec.normal);
        reflected = Vec3::unit_vector(&reflected) + self.fuzz * vec3::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, reflected, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

// Dielectric  透明电介质材质（折射）
#[derive(Debug, Clone, Default)]
pub struct Dielectric {
    pub ir: f64, // 折射指数 (Index of Refraction)
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    pub fn reflectance(cosine: f64, refraction_idx: f64) -> f64 {
        let r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0); // 完全透射，无颜色衰减

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_theta = refraction_ratio * sin_theta > 1.0;
        let dirc = if cannot_theta
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rtweekend::random_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        *scattered = Ray::new(rec.p, dirc, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { emit: texture }
    }

    pub fn new_from_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false // 不进行散射
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { tex: texture }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, vec3::random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}
