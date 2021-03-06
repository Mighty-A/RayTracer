use crate::aabb::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::vec3::Point;
use crate::vec3::Vec3;
use std::sync::Arc;
use std::vec;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point, // the point where ray hit surface
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(m: Arc<dyn Material>) -> Self {
        Self {
            p: Point::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: m,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (r.dire * *outward_normal) < 0.0; // ray into the surface
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: c,
            radius: r,
            mat_ptr: m,
        }
    }
}

pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    *u = 1.0 - (phi + PI) / (2.0 * PI);
    *v = (theta + PI / 2.0) / PI;
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dire.squared_length();
        let half_b = oc * r.dire;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                get_sphere_uv(
                    &((rec.p - self.center) / self.radius),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                get_sphere_uv(
                    &((rec.p - self.center) / self.radius),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            _min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            _max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        };
        true
    }
}

pub struct HittableList {
    pub objects: vec::Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec::Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::clone(rec);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = AABB::new(&Point::ones(), &Point::ones());
        let mut first_box = true;
        for object in &self.objects {
            if !(object.bounding_box(t0, t1, &mut temp_box)) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }
        true
    }
}

pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(cen0: Point, cen1: Point, t0: f64, t1: f64, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn center(&self, time: f64) -> Point {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center(r.tm);
        let a = r.dire.squared_length();
        let half_b = oc * r.dire;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center(r.tm)) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center(r.tm)) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB {
            _min: self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            _max: self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            _min: self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            _max: self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        };
        *output_box = surrounding_box(&box0, &box1);
        true
    }
}

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: &Vec3) -> Self {
        Self {
            ptr: p,
            offset: *displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray {
            orig: r.orig - self.offset,
            dire: r.dire,
            tm: r.tm,
        };
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(t0, t1, output_box) {
            return false;
        }
        *output_box = AABB::new(
            &(output_box._min + self.offset),
            &(output_box._max + self.offset),
        );
        true
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut min = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point::new(-INFINITY, -INFINITY, -INFINITY);
        let mut bbox = AABB::new(&Point::ones(), &Point::ones());
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox._max.x + (1 - i) as f64 * bbox._min.x;
                    let y = j as f64 * bbox._max.y + (1 - j) as f64 * bbox._min.y;
                    let z = k as f64 * bbox._max.z + (1 - k) as f64 * bbox._min.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        let bbox = AABB::new(&min, &max);
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut orig = r.orig;
        let mut dire = r.dire;

        orig[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        orig[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        dire[0] = self.cos_theta * r.dire[0] - self.sin_theta * r.dire[2];
        dire[2] = self.sin_theta * r.dire[0] + self.cos_theta * r.dire[2];

        let rotated_r = Ray {
            orig,
            dire,
            tm: r.tm,
        };

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}
