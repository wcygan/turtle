use std::ops::{Add, Sub};

use cgmath::{Angle, Deg, InnerSpace, Vector2};
use image::{ImageBuffer, RgbImage};
use rand::{Rng, RngCore};
use rand::prelude::ThreadRng;

use crate::arguments::Arguments;

pub mod blobs;
pub mod circle;
pub mod julia_fractal;
pub mod mandelbrot_fractal;
pub mod square;
pub mod pixels;
pub mod squiggly;
pub mod airbrush;
pub mod tree;
pub mod snowflake;
pub mod diamond;

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
/// Determines if the point (x, y) is within the circle centered at (center_x, center_y)
///
fn xy_within_radius_from_center(x: u32, y: u32, center_x: u32, center_y: u32, radius: u32) -> bool {
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
/// Fetch a point on the edge of a rectangle formed by points (0, 0) to (args.size, args.size)
///
fn random_edge_point(args: &mut Arguments) -> (u32, u32) {
    let random_number = args.rng.next_u32() % args.size;
    match args.rng.next_u64() % 4 {
        0 => {
            (0, random_number)
        }
        1 => {
            (random_number, 0)
        }
        2 => {
            (args.size, random_number)
        }
        3 => {
            (random_number, args.size)
        }
        _ => {
            panic!("an integer modulo 4 shouldn't result in a number other than 0, 1, 2, or 3")
        }
    }
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

///
/// Determines if the point (x, y) resides in the rectangle from point (0..width, 0..height)
///
fn point_is_in_rectangle(x: i32, y: i32, width: u32, height: u32) -> bool {
    let x_is_valid = x >= 0 && x < width as i32;
    let y_is_valid = y >= 0 && y < height as i32;
    x_is_valid && y_is_valid
}

///
/// Creates a random angle in the range [0, 360)
///
fn random_angle(rng: &mut ThreadRng) -> Deg<f64> {
    Deg((rng.next_u64() % 360) as f64)
}

///
/// Gets the angle from (x1, y1) to (x2, y2)
///
fn angle_between_points_mirrored_y(x1: u32, y1: u32, x2: u32, y2: u32) -> Deg<f64> {
    let v1 = Vector2::new(x1 as f64, (y1 as i32 * -1) as f64);
    let v2 = Vector2::new(x2 as f64, (y2 as i32 * -1) as f64);
    let rad = v1.angle(v2);
    Deg::from(rad)
}

///
/// Randomly permutes an angle
///
fn randomly_permute_angle(angle: Deg<f64>, limiter: u64, rng: &mut ThreadRng) -> Deg<f64> {
    let degrees_to_add = Deg((rng.next_u64() % limiter) as f64).normalize();
    match rng.next_u64() % 2 == 0 {
        true => {
            angle.add(degrees_to_add).normalize()
        }
        false => {
            angle.sub(degrees_to_add).normalize()
        }
    }
}

///
/// Returns a location one unit away from (x, y) in the direction of an angle
///
fn move_point_one_unit(x: f64, y: f64, angle: Deg<f64>) -> (f64, f64) {
    let (vx, vy) = (angle.cos() * angle.cos().abs(), angle.sin() * angle.sin().abs());
    (x + vx, y + vy)
}

///
/// Fetches a Vec of all neighboring points within "depth" distance from the starting point (x, y)
///
fn neighboring_points_within_depth(depth: i32, x: i32, y: i32, w: u32, h: u32) -> Vec<(i32, i32)> {
    let mut pts = vec![];

    for i in -depth..depth {
        for j in -depth..depth {
            if i == 0 && j == 0 {
                continue;
            }

            let pt = (x + i, y + j);

            if pt.0 < 0 || pt.1 < 0 {
                continue;
            }

            if point_is_in_rectangle(pt.0, pt.1, w, h) {
                pts.push(pt);
            }
        }
    }

    pts
}

///
/// Prune most of the points in a collection away
/// (3% chance of surviving the pruning step)
///
fn prune_points(pts: Vec<(i32, i32)>, rng: &mut ThreadRng) -> Vec<(i32, i32)> {
    let mut selected = vec![];

    for pt in pts {
        match (rng.next_u64() % 33) == 0 {
            true => {
                selected.push(pt)
            }
            false => {}
        }
    }

    selected
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn coordinate_conversion() {
        let max = 20;
        let idx = 193;
        let (x, y) = crate::algorithms::index_to_coordinates(idx, max);
        let new_idx = crate::algorithms::coordinates_to_index(x, y, max);
        assert_eq!(idx, new_idx);
    }

    #[test]
    fn squared() {
        for num in 1..5 {
            let sq = num * num;
            assert_eq!(sq, crate::algorithms::squared(num as i32));
        }
    }

    #[test]
    fn random_angle_is_normalized() {
        for _i in 0..3 {
            let angle = random_angle(&mut thread_rng());
            let degrees = angle.0;
            assert!(degrees < 360.0);
            assert!(degrees >= 0.0);
        }
    }

    #[test]
    fn permuted_angle_is_normalized() {
        let rng = &mut thread_rng();
        for _i in 0..3 {
            let angle = random_angle(rng);
            let angle = randomly_permute_angle(angle, 50, rng);
            let degrees = angle.0;
            assert!(degrees < 360.0);
            assert!(degrees >= 0.0);
        }
    }

    #[test]
    fn image_dimensions() {
        let (ten, twenty) = (10, 20);
        let image: RgbImage = ImageBuffer::new(ten as u32, twenty as u32);
        let (w, h) = image.dimensions();
        assert!(w == ten && h == twenty)
    }


    #[test]
    fn points_inside_rectangle() {
        let (w, h) = (10, 10);
        let pts = vec![(0, 0), (5, 5), (9, 9)];
        for (x, y) in pts {
            assert!(point_is_in_rectangle(x, y, w, h))
        }
    }

    #[test]
    fn points_outside_rectangle() {
        let (w, h) = (10, 10);
        let pts = vec![(20, 0), (5, 10), (10, 10)];
        for (x, y) in pts {
            assert!(!point_is_in_rectangle(x, y, w, h))
        }
    }
}
