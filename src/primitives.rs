use crate::linmath::Vector;
use crate::raytracing::Ray;

pub trait Primitives {
    fn normal(&self ,intersection: Vector) -> Vector;
    fn ray_intersect(&self, ray: &Ray) -> f64;
    fn albedo(&self) -> Vector;
}