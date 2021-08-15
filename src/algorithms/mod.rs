use image::RgbImage;
use rand::Rng;

use crate::arguments::Arguments;

pub mod circle;
pub mod julia_fractal;
pub mod square;

pub trait Create {
    fn create(args: &mut Arguments) -> RgbImage;
}

pub fn random_color(args: &mut Arguments) -> [u8; 3] {
    [
        args.rng.gen_range(0..255) as u8,
        args.rng.gen_range(0..255) as u8,
        args.rng.gen_range(0..255) as u8,
    ]
}
