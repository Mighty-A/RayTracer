use crate::ray::*;
use crate::vec3::*;

#[derive(Clone)]
pub struct AABB {
    pub _min: Point,
    pub _max: Point,
}

impl AABB {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self { _min: *a, _max: *b }
    }

    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dire[a];
            let mut t0 = (self._min[a] - r.orig[a]) * inv_d;
            let mut t1 = (self._max[a] - r.orig[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t0 = if t0 > tmin { t0 } else { tmin };
            t1 = if t1 < tmax { t1 } else { tmax };
            if t1 <= t0 {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point {
        x: box0._min.x.min(box1._min.x),
        y: box0._min.y.min(box1._min.y),
        z: box0._min.z.min(box1._min.z),
    };

    let big = Point {
        x: box0._max.x.max(box1._max.x),
        y: box0._max.y.max(box1._max.y),
        z: box0._max.z.max(box1._max.z),
    };

    AABB {
        _min: small,
        _max: big,
    }
}
