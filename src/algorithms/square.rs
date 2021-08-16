use image::RgbImage;
use rayon::prelude::*;

use crate::algorithms::{new_image_buffer, random_color, Create, RGB_CHUNK_SIZE};
use crate::arguments::Arguments;

pub struct Square {}

impl Create for Square {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let color = random_color(args);
        image.par_chunks_mut(RGB_CHUNK_SIZE).for_each(|p| {
            p.copy_from_slice(&color);
        });
        image
    }
}
