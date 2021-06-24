#![allow(dead_code)]

use crate::hdr::load_hdr;
use crate::image::Image;
use crate::linmath::Vector;
use crate::primitives::{Primitive, Sphere};

mod hdr;
mod image;
mod linmath;
mod primitives;
mod raytracing;
mod render;

fn main() {
    let mut primitives: Vec<Box<dyn Primitive>> = vec![Box::new(Sphere::new(
        Vector::new(0.0, 0.0, 1.0),
        0.35,
        Vector::new(1.0, 1.0, 1.0),
    ))];
    let environment_map = load_hdr("comfy_cafe_16k.hdr");
    let output = render::render(&mut primitives, &environment_map);
    output.save("image.png");
}
