use crate::image::Image;
use crate::linmath::Vector;
use anyhow::{bail, Result};
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Header {
    pub width: usize,
    pub height: usize,
}

pub fn load_hdr<T: AsRef<Path>>(path: T) -> Image {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let header = Header::parse_header(&mut reader).unwrap();

    let mut image = Image::new(header.width, header.height);

    for y in 0..header.height {
        unpack_rle_scanline(y, &mut reader, &mut image).unwrap();
    }

    image
}

pub fn unpack_rle_scanline<B: BufRead>(y: usize, reader: &mut B, image: &mut Image) -> Result<()> {
    let mut red = vec![0; image.width];
    let mut green = vec![0; image.width];
    let mut blue = vec![0; image.width];
    let mut exp = vec![0; image.width];

    let new_rle_indicator = reader.read_u16::<BigEndian>()?;

    if new_rle_indicator != 0x0202 {
        bail!("Bad rle indicator {}", new_rle_indicator);
    }

    let scanline_width = reader.read_u16::<BigEndian>()?;

    if scanline_width as usize != image.width {
        bail!("Bad scanline width {}", scanline_width);
    }

    for i in 0..4 {
        let mut x = 0;
        let color = [&mut red, &mut green, &mut blue, &mut exp];

        while x < image.width {
            let mut count = reader.read_u8().unwrap();

            if count > 128 {
                count &= 0x7F;
                let value = reader.read_u8().unwrap();

                for _ in 0..count {
                    color[i][x] = value;
                    x += 1;
                }
            } else {
                for _ in 0..count {
                    color[i][x] = reader.read_u8().unwrap();
                    x += 1;
                }
            }
        }
    }

    for x in 0..image.width {
        let color = decode_rgbe(red[x], green[x], blue[x], exp[x]);
        image.set_pixel(x, y, color);
    }

    Ok(())
}

pub fn decode_rgbe(r: u8, g: u8, b: u8, e: u8) -> Vector {
    let diff = 128.0 + 8.0;
    let exp = 2.0_f64.powf(e as f64 - diff);
    let r_decoded = r as f64 * exp;
    let g_decoded = g as f64 * exp;
    let b_decoded = b as f64 * exp;

    Vector {
        x: r_decoded,
        y: g_decoded,
        z: b_decoded,
    }
}

impl Header {
    pub fn parse_header<B: BufRead>(reader: &mut B) -> Result<Self> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        if buf.trim() != "#?RADIANCE" {
            bail!("{} is not a .hdr file", buf);
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
                bail!("Does not match 32-bit_rle_rgbe format");
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
