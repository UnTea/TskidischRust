use crate::image::Image;
use crate::linmath::{radians, Vector};
use crate::primitives::Primitive;
use crate::raytracing::{trace_ray, Ray};
use rand::Rng;

const WIDTH: usize = 1024;
const HEIGHT: usize = 780;
const SAMPLE_COUNT: usize = 1;
const FIELD_OF_VIEW: f64 = 120.0;

pub fn render(primitives: &mut [Box<dyn Primitive>], environment_map: &Image) -> Image {
    let mut image = Image::new(WIDTH, HEIGHT);

    for y in 0..image.height / 100 + 1 {
        for x in 0..image.width / 100 + 1 {
            tile(primitives, environment_map, &mut image, x, y)
        }
    }

    image
}

fn tile(
    primitives: &mut [Box<dyn Primitive>],
    environment_map: &Image,
    image: &mut Image,
    tile_x: usize,
    tile_y: usize,
) {
    let aspect_ratio = (image.width as f64) / (image.height as f64);
    let mut random = rand::thread_rng();

    for relative_x in 0..100 {
        for relative_y in 0..100 {
            let x = tile_x * 100 + relative_x;
            let y = tile_y * 100 + relative_y;

            if x >= image.width || y >= image.height {
                continue
            }

            let mut sum = Vector::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLE_COUNT {
                let u = 2.0 * ((x as f64) + random.gen::<f64>()) / (image.width as f64) - 1.0;
                let v = -(2.0 * ((y as f64) + random.gen::<f64>()) / (image.height as f64) - 1.0);

                let film_u = u * f64::tan(radians(FIELD_OF_VIEW) / 2.0) * aspect_ratio;
                let film_v = v * f64::tan(radians(FIELD_OF_VIEW) / 2.0);

                let direction = Vector {
                    x: film_u,
                    y: film_v,
                    z: 1.0,
                }
                .norm();

                let ray = Ray {
                    direction,
                    origin: Vector::new(0.0, 0.0, 0.0),
                };

                let color = trace_ray(primitives, &ray, environment_map, &mut random);
                sum = sum + color;
            }

            image.set_pixel(x, y, sum / (SAMPLE_COUNT as f64));
        }
    }
}
