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
