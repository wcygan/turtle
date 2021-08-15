use crate::algorithms::{Create, Pixel};
use crate::arguments::Arguments;
use image::{ImageBuffer, Rgb, RgbImage};
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);

        let color = Rgb::from([
            args.rng.gen_range(0..255) as u8,
            args.rng.gen_range(0..255) as u8,
            args.rng.gen_range(0..255) as u8,
        ]);

        let coordinates = (0..args.size)
            .cartesian_product(0..args.size)
            .collect::<Vec<(u32, u32)>>();

        let pixels: Vec<Pixel> = coordinates
            .into_par_iter()
            .filter(|coordinate| is_valid_point(coordinate.0, coordinate.1, args.size))
            .map(|coordinate| Pixel::new(coordinate.0, coordinate.1, color))
            .collect::<Vec<Pixel>>();

        for pixel in pixels {
            image.put_pixel(pixel.x, pixel.y, pixel.rgb);
        }

        image
    }
}

/// Determines the point is within the circle
fn is_valid_point(x: u32, y: u32, diameter: u32) -> bool {
    let (x, y) = (x as i64, y as i64);
    let radius = (diameter as i64 / 2) - 1;
    squared(radius) >= squared(radius - x) + squared(radius - y)
}

/// Squares an i64
fn squared(n: i64) -> i64 {
    n * n
}
