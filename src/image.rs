use crate::linmath::Vector;
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Image {
    pub pixels: Vec<Vector>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            pixels: vec![Vector::new(0.0, 0.0, 0.0); width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vector) {
        self.pixels[x + y * self.width] = color
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vector {
        self.pixels[x + y * self.width]
    }

    pub fn get_pixel_uv(&self, u: f64, v: f64) -> Vector {
        let x = (self.width as f64 * (1.0 - u)) as usize;
        let y = (self.height as f64 * (1.0 - v)) as usize;
        self.get_pixel(x, y)
    }

    pub fn get_pixel_by_spherical_coordinates(&self, phi: f64, theta: f64) -> Vector {
        let u = (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / (PI);
        self.get_pixel_uv(u, v)
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) {
        let mut buffer = vec![0; 3 * self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let vec = self.get_pixel(x, y).pow(1.0 / 2.2).clamp(0.0, 1.0);
                buffer[3 * (x + y * self.width) + 0] = (vec.x * 255.0) as u8;
                buffer[3 * (x + y * self.width) + 1] = (vec.y * 255.0) as u8;
                buffer[3 * (x + y * self.width) + 2] = (vec.z * 255.0) as u8;
            }
        }

        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&buffer).unwrap();
    }
}

fn aces_film(color: Vector) -> Vector {
    let a = 2.51;
    let b = Vector::new(0.03, 0.03, 0.03);
    let c = 2.43;
    let d = Vector::new(0.59, 0.59, 0.59);
    let e = Vector::new(0.14, 0.14, 0.14);
    let nominator = color * (color * a + b);
    let denominator = color * (color * c + d) + e;
    nominator / denominator
}
