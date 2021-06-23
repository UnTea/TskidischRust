use crate::image::Image;
use crate::linmath::Vector;
use crate::primitives::Primitives;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::{Add, Mul, Neg};

pub struct Ray {
    pub direction: Vector,
    pub origin: Vector,
}

impl Ray {
    fn point_at(&self, t: f64) -> Vector {
        self.origin.add(self.direction.mul(t))
    }
}

fn random_vector_in_hemisphere(normal: Vector, random: &mut ThreadRng) -> Vector {
    loop {
        let random_vector = Vector {
            x: random.gen(),
            y: random.gen(),
            z: random.gen(),
        };

        let random_vector = random_vector.mul(2.0).add(Vector::splat(-1.0));

        if random_vector.dot(random_vector) > 1.0 {
            continue;
        }

        if random_vector.dot(normal) >= 0.0 {
            return random_vector.norm();
        }

        return random_vector.neg().norm();
    }
}

fn trace_ray(
    primitives: &mut [Box<dyn Primitives>],
    ray: &Ray,
    environment_map: Image,
    random: &mut ThreadRng,
) -> Vector {
    let (primitive, t) = find_intersect(primitives, ray);

    if t == f64::MAX {
        let phi = f64::atan2(ray.direction.z, ray.direction.x);
        let omega =
            f64::sqrt(ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z);
        let theta = f64::atan2(ray.direction.y, omega);

        environment_map.get_pixel_by_spherical_coordinates(phi, theta);
    }

    let ray = Ray {
        direction: random_vector_in_hemisphere(primitive.normal(ray.point_at(t)), random),
        origin: ray.point_at(t),
    };

    let color = primitive
        .albedo()
        .mul(trace_ray(primitives, &ray, environment_map, random));

    color
}

fn find_intersect<'a>(
    primitive: &'a [Box<dyn Primitives>],
    ray: &Ray,
) -> (&'a dyn Primitives, f64) {
    let mut min_t = f64::MAX;
    let mut index: usize = 0;

    for i in 0..primitive.len() {
        let t = primitive[i].ray_intersect(ray);

        if t == -1.0 {
            continue;
        }

        if t < min_t {
            min_t = t;
            index = i;
        }
    }

    (&*primitive[index], min_t)
}
