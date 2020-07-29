mod aabb;
mod aarect;
mod box6;
mod bvh;
mod camera;
mod color;
mod hittable;
mod material;
mod perlin;
mod ray;
mod rtweekend;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

pub use aarect::*;
pub use box6::*;
pub use bvh::*;
pub use camera::*;
pub use color::{ray_color, write_color};
pub use hittable::*;
pub use material::*;
pub use ray::Ray;
pub use rtweekend::*;
pub use std::sync::mpsc::channel;
pub use std::thread;
pub use texture::*;
pub use threadpool::ThreadPool;
pub use vec3::Color;
pub use vec3::Point;
pub use vec3::Vec3;

pub struct ThreadTemp {
    pub x: u32,
    pub color: Vec<[u8; 3]>,
}
fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}
fn main() {
    // Image
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = random_scene();

    // Camera
    let mut aspect_ratio = 3.0 / 2.0;
    let mut lookfrom = Point::new(13.0, 2.0, 3.0);
    let mut lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut dist_to_focus = 10.0;
    let mut aperture = 0.0;
    let mut vfov = 40.0;
    let background;
    let mut samples_per_pixel = 64;

    let scene = 7;
    match scene {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(26.0, 3.0, 6.0);
            lookat = Point::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = light_demo();
            lookfrom = Point::new(13.0, 5.0, 10.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            dist_to_focus = 15.0;
            aperture = 0.4;
            vfov = 40.0;
            background = Color::new(0.0, 0.0, 0.0);
            samples_per_pixel = 300;
        }
        5 => {
            world = two_perlin_spheres();
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            background = Color::new(0.5, 0.8, 0.8);
        }
        6 => {
            world = earth();
            lookfrom = Point::new(13.0, 2.0, 3.0);
            lookat = Point::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            background = Color::new(0.5, 0.8, 0.8);
        }
        7 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            samples_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point::new(278.0, 278.0, -800.0);
            lookat = Point::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            background = Color::new(0.0, 0.0, 0.0);
        }
    };

    let width: u32 = 600;
    let height: u32 = (width as f64 / aspect_ratio) as u32;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        CameraInfo {
            vfov,
            aspect_ratio,
            aperture,
            focus_dist: dist_to_focus,
        },
        0.0,
        1.0,
    );

    // Render
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let bar = ProgressBar::new(width as u64);

    //let pixel_color: [[Color; width as usize]; height as usize] = [[Color::new(0.0, 0.0, 0.0); width as usize]; height as usize];

    println!("width:{} height:{}", width, height);

    let thread_num = if is_ci() { 2 } else { 8 };

    let (tx, rx) = channel();

    for i in 0..thread_num {
        let start = i * width / thread_num;
        let end = (i + 1) * width / thread_num;

        let _tx = tx.clone();
        let _world = world.clone();
        let _cam = cam.clone();
        thread::spawn(move || {
            for x in start..end {
                let mut temp = ThreadTemp { x, color: vec![] };
                for y in 0..height {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = (x as f64 + random_double(0.0, 1.0)) / (width - 1) as f64;
                        let v =
                            ((height - y) as f64 + random_double(0.0, 1.0)) / (height - 1) as f64;
                        let r = _cam.get_ray(u, v);
                        pixel_color += ray_color(&r, &background, &_world, MAX_DEPTH);
                    }
                    let mut r = pixel_color.x;
                    let mut g = pixel_color.y;
                    let mut b = pixel_color.z;

                    let scale = 1.0 / samples_per_pixel as f64;
                    r = (scale * r).sqrt();
                    g = (scale * g).sqrt();
                    b = (scale * b).sqrt();

                    temp.color.push([
                        (clamp(r, 0.0, 0.999) * 255.0) as u8,
                        (clamp(g, 0.0, 0.999) * 255.0) as u8,
                        (clamp(b, 0.0, 0.999) * 255.0) as u8,
                    ]);
                }
                _tx.send(temp).expect("failed to send");
            }
        });
    }
    for receive in rx.iter().take(width as usize) {
        let x = receive.x;
        //print!("{}\n", x);
        for y in 0..height {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb(receive.color[y as usize]);
        }
        bar.inc(1);
    }
    img.save("output/test.png").unwrap();
    bar.finish();
}

pub fn random_scene() -> BVHNode {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian { albedo: checker });
    world.add(Arc::new(Sphere::new(
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
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
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    BVHNode::new(&mut world, 0.0, 0.1)
}

pub fn simple_light() -> BVHNode {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian { albedo: checker });

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        ground_material,
    )));

    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        4.0, 4.0, 4.0,
    )))));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    BVHNode::new(&mut world, 0.0, 1.0)
}

pub fn light_demo() -> BVHNode {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian { albedo: checker });
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -15..15 {
        for b in -15..15 {
            let choose_mat = random_double(0.0, 1.0);
            let r = random_double(0.1, 0.4);
            let center = Point::new(
                a as f64 / 1.5 + 0.25 * random_double(0.0, 1.0),
                r,
                b as f64 / 1.5 + 0.25 * random_double(0.0, 1.0),
            );

            if (center - Point::new(0.0, 0.0, 0.0)).length() > 2.0
                && (center - Point::new(2.0, 0.0, 0.0)).length() > 1.3
                && (center - Point::new(-2.0, 0.0, 0.0)).length() > 1.3
            {
                if choose_mat < 0.2 {
                    // diffuse
                    let albedo = Vec3::elemul(Color::random(0.0, 1.0), Color::random(0.0, 1.0));
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, r, sphere_material)));
                } else if choose_mat < 0.7 {
                    let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(
                        Color::random(0.0, 1.0),
                    ))));
                    world.add(Arc::new(Sphere::new(center, r, difflight)))
                } else if choose_mat < 0.85 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, r, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, r, sphere_material)));
                }
            }
        }
    }
    let difflight = Arc::new(DiffuseLight::new(Arc::new(CheckerTexture::new(
        Color::new(1.0, 0.5, 0.44),
        Color::new(1.0, 0.64, 0.0),
    ))));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.9, 0.0),
        0.9,
        difflight,
    )));

    let material1 = Arc::new(Dielectric::new(1.5));
    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::new(-2.0, 0.65, 0.0),
        0.65,
        material3,
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(2.0, 0.65, 0.0),
        0.65,
        material1,
    )));
    BVHNode::new(&mut world, 0.0, 0.1)
}

fn two_perlin_spheres() -> BVHNode {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    BVHNode::new(&mut world, 0.0, 0.1)
}

fn earth() -> BVHNode {
    let earth_texture = Arc::new(ImageTexture::new("image_texture/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

    BVHNode::new(&mut world, 0.0, 1.0)
}

fn cornell_box() -> BVHNode {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        15.0, 15.0, 15.0,
    )))));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Box6::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Arc::new(Box6::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 165.0, 165.0),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));

    world.add(box2);
    BVHNode::new(&mut world, 0.0, 1.0)
}
