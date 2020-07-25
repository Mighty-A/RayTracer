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
pub use hittable::{HitRecord, Hittable, HittableList, MovingSphere, Sphere};
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;
pub use rtweekend::{random_double, INFINITY, PI};
pub use vec3::Color;
pub use vec3::Point;
pub use vec3::Vec3;
fn main() {
    // Image
    const RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;
    const SAMPLE_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

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

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point::new(
                a as f64 + 0.9 * random_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_double(0.0, 1.0),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::elemul(Color::random(0.0, 1.0), Color::random(0.0, 1.0));
                    let sphere_material = Arc::new(Lambertian::new(&albedo));
                    let center2 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}
