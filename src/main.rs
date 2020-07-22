mod hittable;
mod ray;
mod rtweekend;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use hittable::{HitRecord, Hittable, HittableList, Sphere};
pub use ray::Ray;
pub use rtweekend::{INFINITY, PI};
pub use vec3::Color;
pub use vec3::Point;
pub use vec3::Vec3;
fn main() {
    // Image
    const RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
    // Camera
    let viewport_height = 2.0;
    let viewport_width = RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0); // 水平方向的宽度向量
    let vertical = Vec3::new(0.0, viewport_height, 0.0); // 竖直方向的高度向量
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let bar = ProgressBar::new(HEIGHT as u64);

    //let pixel_color: [[Color; WIDTH as usize]; HEIGHT as usize] = [[Color::new(0.0, 0.0, 0.0); WIDTH as usize]; HEIGHT as usize];

    println!("width:{} height:{}", WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let u = x as f64 / (WIDTH - 1) as f64;
            let v = (HEIGHT - y) as f64 / (HEIGHT - 1) as f64;
            let r = Ray {
                orig: origin,
                dire: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let pixel_color = ray_color(&r, &world);
            write_color(&pixel_color, &mut img, x, y);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn write_color(color: &Color, img: &mut RgbImage, x: u32, y: u32) {
    //println!("R:{} G:{} B:{}", color.x, color.y, color.z);
    let pixel = img.get_pixel_mut(x, y);
    *pixel = image::Rgb([
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    ]);
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = r.dire.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
