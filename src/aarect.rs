use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;
pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.z) / r.dire.z;
        if t < t0 || t > t1 {
            return false;
        }
        let x = r.orig.x + t * r.dire.x;
        let y = r.orig.y + t * r.dire.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            &Point::new(self.x0, self.y0, self.k - 0.0001),
            &Point::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}
