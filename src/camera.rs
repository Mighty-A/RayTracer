use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3::{random_in_unit_disk, Point, Vec3};

pub struct Camera {
    orig: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    //w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (Vec3::cross(vup, w)).unit();
        let v = Vec3::cross(w, u);
        let orig = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = orig - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        Self {
            u,
            v,
            //w,
            orig,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: aperture / 2.0,
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
        }
    }
}
