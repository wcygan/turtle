use crate::algorithms::Create;
use crate::arguments::Arguments;
use image::{Rgb, RgbImage};
use rand::Rng;

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);
        let pixel = Rgb([
            args.rng.gen_range(0..255),
            args.rng.gen_range(0..255),
            args.rng.gen_range(0..255),
        ]);

        // TODO @wcygan: parallelize this with rayon

        todo!("Create circle algorithm")
    }
}
