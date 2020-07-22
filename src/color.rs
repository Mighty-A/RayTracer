use crate::hittable;
use crate::ray::Ray;
pub use crate::rtweekend::{clamp, INFINITY, PI};
use crate::vec3::Color;
pub use hittable::{HitRecord, Hittable, HittableList, Sphere};
use image::RgbImage;

pub fn write_color(
    pixel_color: &Color,
    img: &mut RgbImage,
    x: u32,
    y: u32,
    samples_per_pixel: i32,
) {
    //println!("R:{} G:{} B:{}", color.x, color.y, color.z);
    let pixel = img.get_pixel_mut(x, y);
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    *pixel = image::Rgb([
        (clamp(r, 0.0, 0.999) * 255.0) as u8,
        (clamp(g, 0.0, 0.999) * 255.0) as u8,
        (clamp(b, 0.0, 0.999) * 255.0) as u8,
    ]);
}

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = r.dire.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
