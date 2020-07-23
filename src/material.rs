use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, random_unit_vector, reflect, refract, Color};

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
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray {
            orig: rec.p,
            dire: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
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
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray {
            orig: rec.p,
            dire: refracted,
        };
        true
    }
}
