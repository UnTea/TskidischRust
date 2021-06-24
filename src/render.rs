use crate::image::Image;
use crate::linmath::{radians, Vector};
use crate::primitives::Primitive;
use crate::raytracing::{trace_ray, Ray};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

const WIDTH: usize = 1024;
const HEIGHT: usize = 780;
const SAMPLE_COUNT: usize = 1;
const FIELD_OF_VIEW: f64 = 120.0;

//TODO thread pool

pub fn render(primitives: Vec<Arc<dyn Primitive>>, environment_map: Arc<Image>) -> Image {
    let mut image = Arc::new(Image::new(WIDTH, HEIGHT));

    let handle = thread::spawn(move || {
        let primitives = primitives;
        let image = image;

        for y in 0..image.height / 100 + 1 {
            for x in 0..image.width / 100 + 1 {
                tile(primitives.as_ref(), environment_map.as_ref(), image, x, y)
            }
        }
    })
    .join()
    .unwrap();

    image
}

fn tile(
    primitives: &Vec<Arc<dyn Primitive>>,
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
                continue;
            }

            let mut sum = Vector::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLE_COUNT {
                let u = 2.0 * ((x as f64) + random.gen::<f64>()) / (image.width as f64) - 1.0;
                let v = -(2.0 * ((y as f64) + random.gen::<f64>()) / (image.height as f64) - 1.0);

                let film_u = u * f64::tan(radians(FIELD_OF_VIEW) / 2.0) * aspect_ratio;
                let film_v = v * f64::tan(radians(FIELD_OF_VIEW) / 2.0);

                let direction = Vector::new(film_u, film_v, 1.0).norm();
                let ray = Ray::new(direction, Vector::new(0.0, 0.0, 0.0));

                let color = trace_ray(primitives, &ray, environment_map, &mut random);
                sum = sum + color;
            }

            image.set_pixel(x, y, sum / (SAMPLE_COUNT as f64));
        }
    }
}
