use crate::algorithms::Create;
use crate::arguments::Arguments;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;

pub struct Square {}

impl Create for Square {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);
        let pixel = Rgb([
            args.rng.gen_range(0..255),
            args.rng.gen_range(0..255),
            args.rng.gen_range(0..255),
        ]);

        image.pixels_mut().for_each(|p| *p = pixel);

        image
    }
}
