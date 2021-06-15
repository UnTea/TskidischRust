use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::f64::consts::PI;
use crate::linmath::Vector;

struct Image {
    pixels: Vec<Vector>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        Image {
            pixels: vec![Vector::new(0.0, 0.0, 0.0); width * height],
            width,
            height,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Vector) {
        self.pixels[x + y * self.width] = color
    }

    fn get_pixel(&self, x: usize, y: usize) -> Vector {
        self.pixels[x + y + self.width]
    }

    fn get_pixel_uv(&self, u: f64, v: f64) -> Vector {
        let x = (self.width as f64 * (1.0 - u)) as usize;
        let y = (self.height as f64 * (1.0 - v)) as usize;
        self.get_pixel(x, y)
    }

    fn get_pixel_by_spherical_coordinates(&self, phi: f64, theta: f64) -> Vector {
        let u = (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / (PI);
        self.get_pixel_uv(u, v)
    }

    fn save<T: AsRef<Path>>(&self, path: T) {
        let mut buffer = vec![0; 3 * self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                //filmFramebuffer := ACESFilm(img[x+y*width])
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
