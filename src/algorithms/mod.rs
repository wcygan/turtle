use crate::arguments::Arguments;
use image::RgbImage;

pub mod square;

pub trait Create {
    fn create(args: &mut Arguments) -> RgbImage;
}
