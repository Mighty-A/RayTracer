#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Color;
pub use vec3::Vec3;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let bar = ProgressBar::new(HEIGHT as u64);

    //let pixel_color: [[Color; WIDTH as usize]; HEIGHT as usize] = [[Color::new(0.0, 0.0, 0.0); WIDTH as usize]; HEIGHT as usize];

    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    let hour_angle: f64 = std::f64::consts::PI / 3.0;
    let hour_length: f64 = 200.0;
    let min_angle: f64 = std::f64::consts::PI / 3.0 * 2.0 - 0.3;
    let min_length: f64 = 300.0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let pixel_color = Color::new(255.0, 255.0, 255.0);
            write_color(&pixel_color, &mut img, x, y);
            let square = (x as i32 - mid_x as i32) * (x as i32 - mid_x as i32)
                + (y as i32 - mid_y as i32) * (y as i32 - mid_y as i32);
            if square < 400 * 400 && square > 390 * 390 {
                let pixel_color = Color::new(0.0, 0.0, 0.0);
                write_color(&pixel_color, &mut img, x, y);
            }
        }
        bar.inc(1);
    }

    draw_hand(&mut img, hour_angle, hour_length);
    draw_hand(&mut img, min_angle, min_length);
    /*
    drawing::draw_antialiased_line_segment (
        &img,
        (mid_x as i32, mid_y as i32),
        ((mid_x + 20) as i32, (mid_y + 20) as i32),
        image::Rgb([255, 255, 255]),
        drawing::Blend::new(),
    );
    */

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn write_color(color: &Color, img: &mut RgbImage, x: u32, y: u32) {
    //println!("R:{} G:{} B:{}", color.x, color.y, color.z);
    let pixel = img.get_pixel_mut(x, y);
    *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
}

fn draw_hand(img: &mut RgbImage, angle: f64, length: f64) {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;
    for x in 0..WIDTH as i32 {
        let x_range = angle.cos() * length;
        if x_range >= 0.0 && (x as f64 - mid_x as f64) < x_range && (x as f64 - mid_x as f64) > 0.0
        {
            let y: f64 = (x as f64 - mid_x as f64) * angle.tan();
            let y = -y;
            let y = (y + mid_y as f64) as i32;
            for x_tmp in (x - 5)..(x + 5) {
                for y_tmp in (y - (5 - (x_tmp - x).abs()))..(y + (5 - (x_tmp - x).abs())) {
                    let pixel_color = Color::new(0.0, 0.0, 0.0);
                    write_color(&pixel_color, img, x_tmp as u32, y_tmp as u32);
                }
            }
        }
        if x_range <= 0.0 && (x as f64 - mid_x as f64) > x_range && (x as f64 - mid_x as f64) < 0.0
        {
            let y: f64 = (x as f64 - mid_x as f64) * angle.tan();
            let y = -y;
            let y = (y + mid_y as f64) as i32;
            for x_tmp in (x - 5)..(x + 5) {
                for y_tmp in (y - (5 - (x_tmp - x).abs()))..(y + (5 - (x_tmp - x).abs())) {
                    let pixel_color = Color::new(0.0, 0.0, 0.0);
                    write_color(&pixel_color, img, x_tmp as u32, y_tmp as u32);
                }
            }
        }
    }
}
