use crate::image::Image;
use crate::linmath::Vector;
use crate::primitives::Primitive;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Ray {
    pub direction: Vector,
    pub origin: Vector,
}

impl Ray {
    fn point_at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}

pub fn random_vector_in_hemisphere(normal: Vector, random: &mut ThreadRng) -> Vector {
    loop {
        let mut random_vector = Vector::new(random.gen(), random.gen(), random.gen());
        random_vector = random_vector * 2.0 + Vector::splat(-1.0);

        if random_vector.dot(random_vector) > 1.0 {
            continue;
        }

        if random_vector.dot(normal) >= 0.0 {
            return random_vector.norm();
        }

        return -random_vector.norm();
    }
}

pub fn trace_ray(
    primitives: &mut [Box<dyn Primitive>],
    ray: &Ray,
    environment_map: &Image,
    random: &mut ThreadRng,
) -> Vector {
    let intersection = find_intersect(primitives, ray);

    let (primitive, t) = match intersection {
        None => {
            let phi = f64::atan2(ray.direction.z, ray.direction.x);
            let omega =
                f64::sqrt(ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z);
            let theta = f64::atan2(ray.direction.y, omega);

            return environment_map.get_pixel_by_spherical_coordinates(phi, theta);
        }
        Some(tuple) => tuple,
    };

    let ray = Ray::new(
        random_vector_in_hemisphere(primitive.normal(ray.point_at(t)), random),
        ray.point_at(t),
    );

    let color = primitive.albedo() * trace_ray(primitives, &ray, environment_map, random);

    color
}

pub fn find_intersect<'a>(
    primitives: &'a [Box<dyn Primitive>],
    ray: &Ray,
) -> Option<(&'a dyn Primitive, f64)> {
    primitives
        .iter()
        .map(|x| (x.as_ref(), x.ray_intersect(ray)))
        .filter(|(_, t)| *t != -1.0)
        .min_by(|(_, t1), (_, t2)| t1.partial_cmp(&t2).unwrap())
}
