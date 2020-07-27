use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::texture::*;
use crate::vec3::{random_in_unit_sphere, random_unit_vector, reflect, refract, Color};
use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray {
            orig: rec.p,
            dire: scatter_direction,
            tm: r_in.tm,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        Self {
            albedo: *a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&(r_in.dire.unit()), &(rec.normal));
        *scattered = Ray {
            orig: rec.p,
            dire: reflected + random_in_unit_sphere() * self.fuzz,
            tm: r_in.tm,
        };
        *attenuation = self.albedo;
        scattered.dire * rec.normal > 0.0
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ref_idx: ri }
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
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = r_in.dire.unit();

        let cos_theta = (-unit_direction * rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            // total internal reflection
            let reflected = reflect(&unit_direction, &rec.normal);
            *scattered = Ray {
                orig: rec.p,
                dire: reflected,
                tm: r_in.tm,
            };
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_double(0.0, 1.0) < reflect_prob {
            let reflected = reflect(&unit_direction, &rec.normal);
            *scattered = Ray {
                orig: rec.p,
                dire: reflected,
                tm: r_in.tm,
            };
            return true;
        }
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray {
            orig: rec.p,
            dire: refracted,
            tm: r_in.tm,
        };
        true
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
