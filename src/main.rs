mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod rtweekend;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

pub use color::{ray_color, write_color};
pub use hittable::{HitRecord, Hittable, HittableList, Sphere};
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;
pub use rtweekend::{random_double, INFINITY, PI};
pub use vec3::Color;
pub use vec3::Point;
pub use vec3::Vec3;
fn main() {
    // Image
    const RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;
    const SAMPLE_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Dielectric::new(1.5));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam = camera::Camera::new();

    // Render
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let bar = ProgressBar::new(WIDTH as u64);

    //let pixel_color: [[Color; WIDTH as usize]; HEIGHT as usize] = [[Color::new(0.0, 0.0, 0.0); WIDTH as usize]; HEIGHT as usize];

    println!("width:{} height:{}", WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLE_PER_PIXEL {
                let u = (x as f64 + random_double(0.0, 1.0)) / (WIDTH - 1) as f64;
                let v = ((HEIGHT - y) as f64 + random_double(0.0, 1.0)) / (HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&pixel_color, &mut img, x, y, SAMPLE_PER_PIXEL);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
