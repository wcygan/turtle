use image::RgbImage;
use rand::thread_rng;
use rayon::prelude::*;

use crate::algorithms::{Create, new_image_buffer, RGB_CHUNK_SIZE, thread_local_random_color};
use crate::arguments::Arguments;

pub struct Pixels {}

impl Create for Pixels {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        image.par_chunks_mut(RGB_CHUNK_SIZE).for_each(|p| {
            /* color each pixel randomly */
            let color = thread_local_random_color(&mut thread_rng());
            p.copy_from_slice(&color);
        });
        image
    }
}