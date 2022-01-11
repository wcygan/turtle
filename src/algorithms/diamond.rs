use cgmath::Deg;
use image::{Rgb, RgbImage};
use rayon::prelude::*;

use crate::algorithms::{Create, move_point_one_unit, new_image_buffer, point_is_in_rectangle, random_color, RGB_CHUNK_SIZE};
use crate::arguments::Arguments;

pub struct Diamond {}

static STEPS: u32 = 4;

impl Create for Diamond {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let color = random_color(args);

        let initial_length = args.size / 4;
        let midpoint = args.size / 2;
        let color = random_color(args);
        draw_snowflake(args, &mut image, (midpoint, midpoint), color, STEPS, initial_length);

        image
    }
}

fn draw_snowflake(
    args: &mut Arguments,
    image: &mut RgbImage,
    midpoint: (u32, u32),
    color: [u8; 3],
    steps: u32,
    length: u32,
) {
    if steps == 0 {
        return;
    }
    let (w, h) = image.dimensions();

    for direction in 1..9 {
        let (mut x, mut y) = (midpoint.0 as f64, midpoint.1 as f64);
        let angle = Deg(direction as f64 * 45.0);

        for unit in 0..length {
            let (rx, ry) = (x.round() as i32, y.round() as i32);
            if !point_is_in_rectangle(rx, ry, w, h) {
                break;
            }

            image.put_pixel(rx as u32, ry as u32, Rgb(color));
            let pt = move_point_one_unit(x, y, angle);
            x = pt.0;
            y = pt.1;
        }

        let next_length = length / 3;
        let next_step = steps - 1;
        let next_midpoint = (x.round() as u32, y.round() as u32);
        draw_snowflake(args, image, next_midpoint, color, next_step, next_length);
    }
}