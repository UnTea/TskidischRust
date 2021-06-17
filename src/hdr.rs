use crate::image::Image;
use crate::linmath::Vector;
use png::Reader;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub struct Header {
    width: usize,
    height: usize,
}

pub fn load_hdr<T: AsRef<Path>>(path: T) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let header = Header::parse_header(&mut reader).unwrap();
}

pub fn unpack_rle_scanline() {
    todo!()
}

pub fn decode_rgbe() -> Vector {
    todo!()
}

pub fn read_u16() -> u16 {
    todo!()
}

pub fn read_u8() -> u8 {
    todo!()
}

impl Header {
    pub fn parse_header<B: BufRead>(reader: &mut B) -> Result<Self, Box<dyn Error>> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        if buf.trim() != "#?RADIANCE" {
            panic!("{} is not a .hdr file", buf);
        }

        for lined in reader.lines() {
            let lined = lined?;

            if lined.is_empty() {
                break;
            }

            if !lined.starts_with("FORMAT") {
                continue;
            }

            let mut parts = lined.split("=");

            if parts.nth(1).unwrap() != "32-bit_rle_rgbe" {
                panic!("Does not match 32-bit_rle_rgbe format");
            }
        }

        buf.clear();
        reader.read_line(&mut buf)?;

        let mut resolution = buf.split_whitespace();
        let height = resolution.nth(1).unwrap().parse()?;
        let width = resolution.nth(1).unwrap().parse()?;

        Ok(Header { width, height })
    }
}
