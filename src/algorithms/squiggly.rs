


use image::{Rgb, RgbImage};

use rand::prelude::ThreadRng;

use crate::algorithms::{Create, move_point_one_unit, new_image_buffer, point_is_in_rectangle, random_angle, random_color, random_points, randomly_permute_angle};
use crate::arguments::Arguments;

static ANGLE_DIFFERENCE_LIMITER: u64 = 120;

pub struct Squiggly {}

impl Create for Squiggly {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let mut rng = rand::thread_rng();

        let points = random_points(args, 50);
        for (x, y) in points {
            let color = random_color(args);
            draw_squiggly_line(&mut image, x as i32, y as i32, color, &mut rng)
        }

        image
    }
}

pub fn draw_squiggly_line(
    image: &mut RgbImage,
    x: i32,
    y: i32,
    color: [u8; 3],
    rng: &mut ThreadRng,
) {
    let mut x = x as f64;
    let mut y = y as f64;
    let (w, h) = image.dimensions();
    let mut angle = random_angle(rng);

    loop {
        let (rx, ry) = (x.round() as i32, y.round() as i32);

        if !point_is_in_rectangle(rx, ry, w, h) {
            break;
        }

        image.put_pixel(rx as u32, ry as u32, Rgb(color));
        angle = randomly_permute_angle(angle, ANGLE_DIFFERENCE_LIMITER, rng);
        let pt = move_point_one_unit(x, y, angle);
        x = pt.0;
        y = pt.1;
    }
}