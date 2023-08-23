use super::color::Color;
use std::fs;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color::black(); (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn save_ppm(&self, name: &str) {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for pixel in &self.pixels {
            ppm.push_str(&format!(
                "{} {} {}\n",
                (pixel.r * 255.0).round(),
                (pixel.g * 255.0).round(),
                (pixel.b * 255.0).round()
            ));
        }

        fs::write(name, ppm).expect("Could not write image file");
    }

    pub fn save_png(&self, name: &str) {
        use image::{Rgb, RgbImage};

        let mut img = RgbImage::new(self.width, self.height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = self.get_pixel(x, y);
            *pixel = Rgb([
                (color.r * 255.0).round() as u8,
                (color.g * 255.0).round() as u8,
                (color.b * 255.0).round() as u8,
            ]);
        }

        img.save(name).unwrap();
    }
}
