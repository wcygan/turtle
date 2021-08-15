use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

use crate::algorithms::{random_color, Create};
use crate::arguments::Arguments;

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);
        let data = random_color(args);
        image.par_chunks_mut(3).enumerate().for_each(|(i, p)| {
            let x = i as u32 % args.size;
            let y = (i as u32 - x) / args.size;
            if is_valid_point(x, y, args.size) {
                p.copy_from_slice(&data);
            }
        });
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
