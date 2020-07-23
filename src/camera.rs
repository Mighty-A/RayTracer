use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3::Point;
use crate::vec3::Vec3;

pub struct Camera {
    orig: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = (Vec3::cross(vup, w)).unit();
        let v = Vec3::cross(w, u);

        Self {
            orig: lookfrom,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
            lower_left_corner: lookfrom
                - (u * viewport_width) / 2.0
                - (v * viewport_height) / 2.0
                - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            orig: self.orig,
            dire: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.orig,
        }
    }
}
