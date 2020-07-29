use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::texture::*;
use crate::vec3::*;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }
    pub fn new_from_color(b: Arc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_from_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let en_anble_debug: bool = false;
        let debuging: bool = en_anble_debug && random_double(0.0, 0.1) < 0.00001;

        let mut rec1 = HitRecord::new(Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))));
        let mut rec2 = rec1.clone();

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if debuging {
            println!("\nt0={}, t1={}\n", rec1.t, rec2.t);
        }
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t > rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dire.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0, 1.0).log10();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debuging {
            println!(
                "hit_distance = {}\nrec.t = {}\nrec.p = {:?}\n",
                hit_distance, rec.t, rec.p
            );
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // arbitrary
        rec.mat_ptr = self.phase_function.clone();
        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(t0, t1, output_box)
    }
}
