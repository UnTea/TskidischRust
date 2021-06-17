#![allow(dead_code)]

use crate::hdr::{load_hdr, Header};

mod hdr;
mod image;
mod linmath;

fn main() {
    let enviroment_map = load_hdr("wooden_lounge_1k.hdr");
}
