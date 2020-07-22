use crate::vec3::Color;
use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub orig: Vec3,
    pub dire: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dire * t
    }
}

pub fn ray_color(ray: Ray) -> Color {
    let unit_dir = ray.dire.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(255.0, 255.0, 255.0) * (1.0 - t)
        + Color::new(255.0 * 0.5, 255.0 * 0.7, 255.0 * 1.0) * t
}
