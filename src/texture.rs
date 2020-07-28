//use crate::rtweekend::*;
use crate::perlin::*;
use crate::rtweekend::*;
use crate::vec3::*;
use image::GenericImageView;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (p.x * 10.0).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        Color::ones()
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(&(*p * self.scale), 7)).sin())
    }
}

pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    img: image::DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = image::open(filename).unwrap();
        Self {
            width: img.width(),
            height: img.height(),
            img: img,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel_color = self.img.get_pixel(i, j);
        Color::new(
            color_scale * pixel_color[0] as f64,
            color_scale * pixel_color[1] as f64,
            color_scale * pixel_color[2] as f64,
        )
    }
}
