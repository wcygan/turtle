use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

use crate::algorithms::{index_to_coordinates, is_valid_point, random_color, Create};
use crate::arguments::Arguments;

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);
        let data = random_color(args);
        image.par_chunks_mut(3).enumerate().for_each(|(i, p)| {
            let (x, y) = index_to_coordinates(i as u32, args.size);
            if is_valid_point(x, y, args.size) {
                p.copy_from_slice(&data);
            }
        });
        image
    }
}
