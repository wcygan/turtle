use image::{ImageBuffer, RgbImage};
use rand::Rng;

use crate::arguments::Arguments;
use rand::prelude::ThreadRng;

pub mod blobs;
pub mod circle;
pub mod julia_fractal;
pub mod mandelbrot_fractal;
pub mod square;
pub mod pixels;

static RGB_CHUNK_SIZE: usize = 3;

///
/// Creates an RbgImage
///
pub trait Create {
    fn create(args: &mut Arguments) -> RgbImage;
}

///
/// Creates a blank canvas
///
fn new_image_buffer(args: &mut Arguments) -> RgbImage {
    ImageBuffer::new(args.size as u32, args.size as u32)
}

///
/// Generates a random color
///
fn random_color(args: &mut Arguments) -> [u8; 3] {
    [args.rng.gen_range(0..255) as u8, args.rng.gen_range(0..255) as u8, args.rng.gen_range(0..255) as u8]
}

///
/// Generates a random color for ThreadRng
///
fn thread_local_random_color(rng: &mut ThreadRng) -> [u8; 3] {
    [rng.gen_range(0..255) as u8, rng.gen_range(0..255) as u8, rng.gen_range(0..255) as u8]
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
    let x = idx % length;
    let y = idx / length;
    (x, y)
}

///
/// Maps a pair (x, y) in the coordinate plane
/// to an index in the range 0..(length**2)
///
#[allow(dead_code)]
fn coordinates_to_index(x: u32, y: u32, length: u32) -> u32 {
    (y * length) + x
}

///
/// Determines the point is within the circle
///
fn is_valid_point(x: u32, y: u32, center_x: u32, center_y: u32, radius: u32) -> bool {
    let (x, y, center_x, center_y, radius) = (
        x as i32,
        y as i32,
        center_x as i32,
        center_y as i32,
        radius as i32,
    );

    squared(radius) >= squared(center_x - x) + squared(center_y - y)
}

///
/// Squares an i32
///
fn squared(n: i32) -> u32 {
    (n * n) as u32
}

///
/// Finds the max index, args.size**2
///
fn max_index(args: &Arguments) -> u32 {
    squared(args.size as i32)
}

///
/// Selects a random index in the coordinate plane
///
fn random_index(args: &mut Arguments) -> u32 {
    args.rng.gen_range(0..max_index(args))
}

///
/// Creates a Vec<(u32, u32)> of n random points
///
fn random_points(args: &mut Arguments, n: u32) -> Vec<(u32, u32)> {
    (0..n)
        .map(|_| index_to_coordinates(random_index(args), args.size))
        .collect()
}

///
/// Converts a point that is out of bounds to the nearest border point
///
fn convert_if_out_of_bounds(val: i32, max: u32) -> u32 {
    let val = {
        if val < 0 {
            0
        } else {
            val as u32
        }
    };

    if val > max {
        max
    } else {
        val
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn coordinate_conversion() {
        let max = 20;
        let idx = 193;

        // convert from (x, y) to index
        let (x, y) = crate::algorithms::index_to_coordinates(idx, max);

        // convert from index to (x, y)
        let new_idx = crate::algorithms::coordinates_to_index(x, y, max);

        assert_eq!(idx, new_idx);
    }

    #[test]
    fn squared() {
        let vec = [1, 2, 3, 4, 5];
        for num in vec {
            let sq = (num * num);
            assert_eq!(sq, crate::algorithms::squared(num));
        }
    }
}
