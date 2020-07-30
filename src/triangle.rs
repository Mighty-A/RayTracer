use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;
pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point0: Point,
    pub mp: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(point1: Point, point2: Point, point0: Point, mp: Arc<dyn Material>) -> Self {
        Self {
            point1,
            point2,
            point0,
            mp,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let edge1 = self.point1 - self.point0;
        let edge2 = self.point2 - self.point0;

        let pvec = Vec3::cross(r.dire, edge2); // cross P = D X E2

        let det = pvec * edge1; // 行列式（克莱姆

        const EPSILON: f64 = 1e-8;
        if det < EPSILON && det > -EPSILON {
            return false;
        }

        let inv_det = 1.0 / det;

        let tvec = r.orig - self.point0;

        let u = tvec * pvec * inv_det;

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let qvec = Vec3::cross(tvec, edge1);
        let v = r.dire * qvec * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = edge2 * qvec * inv_det;
        if t < t_min || t > t_max {
            return false;
        }

        let outward_normal = Vec3::cross(edge1, edge2).unit();
        rec.set_face_normal(r, &outward_normal);
        rec.u = u;
        rec.v = v;
        rec.t = t;
        rec.p = r.at(t);
        rec.mat_ptr = self.mp.clone();
        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        let min_x = self.point0.x.min(self.point1.x).min(self.point2.x);
        let min_y = self.point0.y.min(self.point1.y).min(self.point2.y);
        let min_z = self.point0.z.min(self.point1.z).min(self.point2.z);
        let max_x = self.point0.x.max(self.point1.x).max(self.point2.x);
        let max_y = self.point0.y.max(self.point1.y).max(self.point2.y);
        let max_z = self.point0.z.max(self.point1.z).max(self.point2.z);

        *output_box = AABB::new(
            &Point::new(min_x, min_y, min_z),
            &Point::new(max_x, max_y, max_z),
        );
        true
    }
}
