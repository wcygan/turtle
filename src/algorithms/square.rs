use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

use crate::algorithms::{random_color, Create};
use crate::arguments::Arguments;

pub struct Square {}

impl Create for Square {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);
        let data = random_color(args);
        image.par_chunks_mut(3).for_each(|p| {
            p.copy_from_slice(&data);
        });
        image
    }
}
