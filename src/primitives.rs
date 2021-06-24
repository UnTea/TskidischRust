use crate::linmath::{Vector, EPSILON};
use crate::raytracing::Ray;

pub trait Primitive {
    fn normal(&self, intersection: Vector) -> Vector;
    fn ray_intersect(&self, ray: &Ray) -> f64;
    fn albedo(&self) -> Vector;
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub albedo: Vector,
}

pub struct Plane {
    pub point: Vector,
    pub normal: Vector,
    pub albedo: Vector,
}

impl Primitive for Sphere {
    fn normal(&self, intersection: Vector) -> Vector {
        intersection - self.center.norm()
    }

    fn ray_intersect(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
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

    fn albedo(&self) -> Vector {
        self.albedo
    }
}

impl Primitive for Plane {
    fn normal(&self, _intersection: Vector) -> Vector {
        self.normal
    }

    fn ray_intersect(&self, ray: &Ray) -> f64 {
        let denominator = self.normal.dot(ray.direction);

        if f64::abs(denominator) > EPSILON {
            let t = (self.point - ray.origin).dot(self.normal) / denominator;

            if t >= EPSILON {
                return t;
            }
        }

        -1.0
    }

    fn albedo(&self) -> Vector {
        self.albedo
    }
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, albedo: Vector) -> Sphere {
        Sphere {
            center,
            radius,
            albedo,
        }
    }
}

impl Plane {
    pub fn new(point: Vector, normal: Vector, albedo: Vector) -> Plane {
        Plane {
            point,
            normal,
            albedo,
        }
    }
}
