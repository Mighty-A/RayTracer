use crate::ray::Ray;
use crate::rtweekend::{degrees_to_radians, random_double};
use crate::vec3::{random_in_unit_disk, Point, Vec3};

pub struct Camera {
    orig: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

pub struct CameraInfo {
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        info: CameraInfo,
        t0: f64, // shutter open
        t1: f64, // shutter close
    ) -> Self {
        let theta = degrees_to_radians(info.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = info.aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (Vec3::cross(vup, w)).unit();
        let v = Vec3::cross(w, u);
        let orig = lookfrom;
        let horizontal = u * info.focus_dist * viewport_width;
        let vertical = v * info.focus_dist * viewport_height;
        let lower_left_corner = orig - horizontal / 2.0 - vertical / 2.0 - w * info.focus_dist;
        Self {
            u,
            v,
            _w: w,
            orig,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: info.aperture / 2.0,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            orig: self.orig + offset,
            dire: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.orig
                - offset,
            tm: random_double(self.time0, self.time1),
        }
    }
}
