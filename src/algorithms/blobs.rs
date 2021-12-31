use image::RgbImage;
use rand::Rng;

use crate::algorithms::{Create, new_image_buffer, random_color, random_points};
use crate::arguments::Arguments;

static RADIUS_LIMITER: u32 = 10;
static NUMBER_OF_POINTS_LIMITER: u32 = 5;

pub struct Blobs {}

impl Create for Blobs {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let points = random_points(args, args.size / NUMBER_OF_POINTS_LIMITER);

        for pt in points {
            let (x, y) = (pt.0, pt.1);
            let color = random_color(args);
            let radius = args.rng.gen_range(0..(args.size / RADIUS_LIMITER));
            crate::algorithms::circle::fill_in_circle(&mut image, x, y, radius, color);
        }

        image
    }
}
