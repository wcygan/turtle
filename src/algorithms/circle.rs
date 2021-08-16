use crate::algorithms::{
    convert_if_out_of_bounds, is_valid_point, new_image_buffer, random_color, Create,
};
use crate::arguments::Arguments;
use image::{Rgb, RgbImage};

pub struct Circle {}

impl Create for Circle {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let color = random_color(args);
        let radius = args.size / 2;
        fill_in_circle(&mut image, radius, radius, radius, color);
        image
    }
}

///
/// Colors every pixel within radius distance from a point (center_x, center_y) in an image
///
pub fn fill_in_circle(
    image: &mut RgbImage,
    center_x: u32,
    center_y: u32,
    radius: u32,
    color: [u8; 3],
) {
    let (w, h) = image.dimensions();

    let (lo_x, hi_x) = (
        convert_if_out_of_bounds(center_x as i32 - radius as i32, 0),
        convert_if_out_of_bounds(center_x as i32 + radius as i32, w),
    );

    let (lo_y, hi_y) = (
        convert_if_out_of_bounds(center_y as i32 - radius as i32, 0),
        convert_if_out_of_bounds(center_y as i32 + radius as i32, h),
    );

    for x in lo_x..hi_x {
        for y in lo_y..hi_y {
            if is_valid_point(x, y, center_x, center_y, radius) {
                image.put_pixel(x, y, Rgb(color));
            }
        }
    }
}
