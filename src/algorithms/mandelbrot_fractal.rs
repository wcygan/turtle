use crate::algorithms::{index_to_coordinates, into_rgb, new_image_buffer, Create, RGB_CHUNK_SIZE};
use crate::arguments::Arguments;
use image::RgbImage;
use rayon::prelude::*;

pub struct MandelbrotFractal {}

impl Create for MandelbrotFractal {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);

        image
            .par_chunks_mut(RGB_CHUNK_SIZE)
            .enumerate()
            .for_each(|(i, p)| todo!());

        image
    }
}
