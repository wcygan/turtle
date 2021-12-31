use std::ops::Add;

use cgmath::{Angle, Deg, Rad, Vector2};
use image::{Rgb, RgbImage};
use rand::{Rng, RngCore, thread_rng};
use rand::prelude::ThreadRng;

use crate::algorithms::{convert_if_out_of_bounds, Create, move_point_one_unit, new_image_buffer, point_is_in_rectangle, random_angle, random_color, random_points, randomly_permute_angle, xy_within_radius_from_center};
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
    mut x: i32,
    mut y: i32,
    color: [u8; 3],
    rng: &mut ThreadRng,
) {
    let (w, h) = image.dimensions();
    let mut angle = random_angle(rng);

    loop {
        if !point_is_in_rectangle(x, y, w, h) {
            break;
        }

        image.put_pixel(x as u32, y as u32, Rgb(color));
        angle = randomly_permute_angle(angle, ANGLE_DIFFERENCE_LIMITER, rng);
        let pt = move_point_one_unit(x, y, angle);
        x = pt.0;
        y = pt.1;
    }
}