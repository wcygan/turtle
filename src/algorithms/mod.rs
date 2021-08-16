use image::RgbImage;
use rand::Rng;

use crate::arguments::Arguments;

pub mod circle;
pub mod julia_fractal;
pub mod square;

static RGB_CHUNK_SIZE: usize = 3;

///
/// Creates an RbgImage
///
pub trait Create {
    fn create(args: &mut Arguments) -> RgbImage;
}

///
/// Generates a random color
///
fn random_color(args: &mut Arguments) -> [u8; 3] {
    [
        args.rng.gen_range(0..255) as u8,
        args.rng.gen_range(0..255) as u8,
        args.rng.gen_range(0..255) as u8,
    ]
}

///
/// Generates a [u8; 3] with red, green, and blue values
///
fn into_rgb(r: u8, g: u8, b: u8) -> [u8; 3] {
    [r, g, b]
}

///
/// Maps an index in the range 0..(length**2)
/// to a pair (x, y) in the coordinate plane
///
fn index_to_coordinates(idx: u32, length: u32) -> (u32, u32) {
    let x = idx as u32 % length;
    let y = (idx as u32 - x) / length;
    (x, y)
}

///
/// Determines the point is within the circle
///
fn is_valid_point(x: u32, y: u32, diameter: u32) -> bool {
    let (x, y) = (x as i64, y as i64);
    let radius = (diameter as i64 / 2) - 1;
    squared(radius) >= squared(radius - x) + squared(radius - y)
}

///
/// Squares an i64
///
fn squared(n: i64) -> i64 {
    n * n
}
