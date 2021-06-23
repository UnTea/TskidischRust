use crate::linmath::{Vector, EPSILON};
use crate::raytracing::Ray;
use std::ops::Sub;

pub trait Primitives {
    fn normal(&self, intersection: Vector) -> Vector;
    fn ray_intersect(&self, ray: &Ray) -> f64;
    fn albedo(&self) -> Vector;
}

pub struct Sphere {
    center: Vector,
    radius: f64,
    albedo: Vector,
}

pub struct Plane {
    point: Vector,
    normal: Vector,
    albedo: Vector,
}

impl Sphere {
    fn new(center: Vector, radius: f64, albedo: Vector) -> Sphere {
        Sphere {
            center,
            radius,
            albedo,
        }
    }

    fn albedo(&self) -> Vector {
        self.albedo
    }

    fn normal(&self, intersection: Vector) -> Vector {
        intersection.sub(self.center).norm()
    }

    fn ray_intersect(&self, ray: &Ray) -> f64 {
        let oc = ray.origin.sub(self.center);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let h = b * b - c;

        if h < 0.0 {
            return -1.0;
        }

        let h = f64::sqrt(h);

        if -b - h > EPSILON {
            return -b - h; // t is -b -h
        }

        if -b + h > EPSILON {
            return -b + h;
        }

        -1.0
    }
}

impl Plane {
    fn new(point: Vector, normal: Vector, albedo: Vector) -> Plane {
        Plane {
            point,
            normal,
            albedo,
        }
    }

    fn albedo(&self) -> Vector {
        self.albedo
    }

    fn normal(&self) -> Vector {
        self.normal
    }

    fn ray_intersect(&self, ray: &Ray) -> f64 {
        let denominator = self.normal.dot(ray.direction);

        if f64::abs(denominator) > EPSILON {
            let t = self.point.sub(ray.origin).dot(self.normal) / denominator;

            if t >= EPSILON {
                return t;
            }
        }

        -1.0
    }
}
