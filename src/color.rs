use crate::hittable;
use crate::material::Lambertian;
use crate::ray::Ray;
pub use crate::rtweekend::{clamp, INFINITY, PI};
use crate::vec3::{Color, Vec3};
pub use hittable::{HitRecord, Hittable, HittableList, Sphere};
use image::RgbImage;
use std::sync::Arc;

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

pub fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new(Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))));

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if !world.hit(r, 0.0001, INFINITY, &mut rec) {
        return *background;
    }
    let mut scattered = Ray {
        orig: Vec3::ones(),
        dire: Vec3::ones(),
        tm: 0.0,
    };
    let mut attenuation = Color::new(0.0, 0.0, 0.0);
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

    if !rec
        .mat_ptr
        .scatter(&r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted
        + Vec3::elemul(
            attenuation,
            ray_color(&scattered, background, world, depth - 1),
        )
}
