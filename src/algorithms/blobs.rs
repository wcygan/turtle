use crate::algorithms::{new_image_buffer, random_color, random_points, Create};
use crate::arguments::Arguments;
use image::RgbImage;
use rand::Rng;

static RADIUS_LIMITER: u32 = 10;
static NUMBER_OF_POINTS_LIMITER: u32 = 5;

pub struct Blobs {}

impl Create for Blobs {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let points = random_points(args, args.size / NUMBER_OF_POINTS_LIMITER);

        /* create a circle of random radius length around the randomly generated point */
        for pt in points {
            let (x, y) = (pt.0, pt.1);
            let color = random_color(args);
            let radius = args.rng.gen_range(0..(args.size / RADIUS_LIMITER));
            crate::algorithms::circle::fill_in_circle(&mut image, x, y, radius, color);
        }

        image
    }
}
