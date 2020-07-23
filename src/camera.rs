use crate::ray::Ray;
use crate::vec3::Point;
use crate::vec3::Vec3;

pub struct Camera {
    orig: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        Self {
            orig: Point::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
            lower_left_corner: Point::new(0.0, 0.0, 0.0)
                - Vec3::new(viewport_width, 0.0, 0.0) / 2.0
                - Vec3::new(0.0, viewport_height, 0.0) / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.orig,
            dire: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.orig,
        }
    }
}
