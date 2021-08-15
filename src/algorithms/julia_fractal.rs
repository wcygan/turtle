use crate::algorithms::Create;
use crate::arguments::Arguments;
use image::RgbImage;

pub struct JuliaFractal {}

impl Create for JuliaFractal {
    fn create(args: &mut Arguments) -> RgbImage {
        todo!("julia fractal")
    }
}
