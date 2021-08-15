use crate::algorithms::Create;
use crate::arguments::Arguments;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        todo!("Create circle algorithm")
    }
}
