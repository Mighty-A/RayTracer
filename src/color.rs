use crate::hittable;
use crate::ray::Ray;
pub use crate::rtweekend::{clamp, INFINITY, PI};
use crate::vec3::{Color, Point, random_in_hemisphere};
pub use hittable::{HitRecord, Hittable, HittableList, Sphere};
use image::RgbImage;

pub fn write_color(
    pixel_color: &Color,
    img: &mut RgbImage,
    pixel_x: u32,
    pixel_y: u32,
    samples_per_pixel: i32,
) {
    let pixel = img.get_pixel_mut(pixel_x, pixel_y);
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    *pixel = image::Rgb([
        (clamp(r, 0.0, 0.999) * 255.0) as u8,
        (clamp(g, 0.0, 0.999) * 255.0) as u8,
        (clamp(b, 0.0, 0.999) * 255.0) as u8,
    ]);
}

pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target: Point = rec.p + random_in_hemisphere(&rec.normal);
        return ray_color(&Ray{orig: rec.p, dire: target - rec.p,}, world, depth - 1) * 0.5;            // recursive
    }
    let unit_direction = r.dire.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
