use crate::aabb::*;
use crate::hittable::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::vec3::*;
use std::cmp::*;
use std::sync::Arc;
use std::vec::Vec;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    _box: AABB,
}

pub fn new_from_list(
    objects: &mut Vec<Arc<dyn Hittable>>,
    start: usize,
    end: usize,
    time0: f64,
    time1: f64,
) -> BVHNode {
    let axis = random_int(0, 2);
    let comparator = {
        if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        }
    };

    let object_span = end - start;

    let mut temp;

    if object_span == 1 {
        temp = BVHNode {
            left: objects[start].clone(),
            right: objects[start].clone(),
            _box: AABB::new(&Point::ones(), &Point::ones()),
        }
    } else if object_span == 2 {
        match comparator(&objects[start], &objects[start + 1]) {
            Ordering::Less => {
                temp = BVHNode {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    _box: AABB::new(&Point::ones(), &Point::ones()),
                }
            }
            Ordering::Greater => {
                temp = BVHNode {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    _box: AABB::new(&Point::ones(), &Point::ones()),
                }
            }
            Ordering::Equal => {
                temp = BVHNode {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    _box: AABB::new(&Point::ones(), &Point::ones()),
                }
            }
        }
    } else {
        objects.as_mut_slice()[start..end].sort_by(comparator);

        let mid = start + object_span / 2;
        temp = BVHNode {
            left: Arc::new(new_from_list(objects, start, mid, time0, time1)),
            right: Arc::new(new_from_list(objects, mid, end, time0, time1)),
            _box: AABB::new(&Point::ones(), &Point::ones()),
        }
    }

    let mut box_left = AABB::new(&Point::ones(), &Point::ones());
    let mut box_right = AABB::new(&Point::ones(), &Point::ones());

    if !temp.left.bounding_box(time0, time1, &mut box_left)
        || !temp.right.bounding_box(time0, time1, &mut box_right)
    {
        panic!();
    }
    temp._box = surrounding_box(&box_left, &box_right);

    temp
}

impl BVHNode {
    pub fn new(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let length = list.objects.len();
        new_from_list(&mut list.objects, 0, length, time0, time1)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self._box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self._box.clone();
        true
    }
}

pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut box_a = AABB::new(&Point::ones(), &Point::ones());
    let mut box_b = AABB::new(&Point::ones(), &Point::ones());

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        panic!();
    }

    if box_a._min[axis] < box_b._min[axis] {
        Ordering::Less
    } else if box_a._min[axis] > box_b._min[axis] {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
