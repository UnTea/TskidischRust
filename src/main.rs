#![allow(dead_code)]

use crate::hdr::load_hdr;

mod hdr;
mod image;
mod linmath;
mod primitives;
mod raytracing;

fn main() {
    let enviroment_map = load_hdr("wooden_lounge_1k.hdr");
}
