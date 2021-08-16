use crate::algorithms::{
    index_to_coordinates, is_valid_point, random_color, Create, RGB_CHUNK_SIZE,
};
use crate::arguments::Arguments;
use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);
        let data = random_color(args);
        image
            .par_chunks_mut(RGB_CHUNK_SIZE)
            .enumerate()
            .for_each(|(i, p)| {
                let (x, y) = index_to_coordinates(i as u32, args.size);
                if is_valid_point(x, y, args.size) {
                    p.copy_from_slice(&data);
                }
            });
        image
    }
}
