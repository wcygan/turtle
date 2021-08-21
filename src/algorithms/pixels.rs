use crate::algorithms::{Create, RGB_CHUNK_SIZE, new_image_buffer, thread_local_random_color};
use crate::arguments::Arguments;
use image::RgbImage;
use rayon::prelude::*;
use rand::thread_rng;

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