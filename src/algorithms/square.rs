use crate::algorithms::{Create, Pixel};
use crate::arguments::Arguments;
use image::{ImageBuffer, Rgb, RgbImage};
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;

pub struct Square {}

impl Create for Square {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);

        let color = Rgb::from([
            args.rng.gen_range(0..255) as u8,
            args.rng.gen_range(0..255) as u8,
            args.rng.gen_range(0..255) as u8,
        ]);

        let coordinates = (0..args.size)
            .cartesian_product(0..args.size)
            .collect::<Vec<(u32, u32)>>();

        let pixels: Vec<Pixel> = coordinates
            .into_par_iter()
            .map(|coordinate| Pixel::new(coordinate.0, coordinate.1, color))
            .collect::<Vec<Pixel>>();

        for pixel in pixels {
            image.put_pixel(pixel.x, pixel.y, pixel.rgb);
        }

        image
    }
}
