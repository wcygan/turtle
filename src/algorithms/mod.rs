use crate::arguments::Arguments;
use image::{Rgb, RgbImage};

pub mod circle;
pub mod square;

pub struct Pixel {
    x: u32,
    y: u32,
    rgb: Rgb<u8>,
}

impl Pixel {
    pub fn new(x: u32, y: u32, rgb: Rgb<u8>) -> Pixel {
        Pixel { x, y, rgb }
    }
}

pub trait Create {
    fn create(args: &mut Arguments) -> RgbImage;
}
