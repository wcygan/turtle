use std::ops::Add;

use cgmath::{Angle, Deg};
use image::{Rgb, RgbImage};
use rand::prelude::ThreadRng;
use rand::RngCore;

use crate::algorithms::{Create, move_point_one_unit, neighboring_points_within_depth, new_image_buffer, point_is_in_rectangle, prune_points, random_angle, random_color, random_points, randomly_permute_angle};
use crate::arguments::Arguments;

static ANGLE_DIFFERENCE_LIMITER: u64 = 180;

pub struct Airbrush {}

impl Create for Airbrush {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let mut rng = rand::thread_rng();

        let points = random_points(args, 100);
        for (x, y) in points {
            let color = random_color(args);
            draw_airbrushed_line(&mut image, x as i32, y as i32, color, &mut rng)
        }

        image
    }
}

pub fn draw_airbrushed_line(
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

        let neighbors = neighboring_points_within_depth(15, x, y, w, h);
        let points_to_color = prune_points(neighbors, rng);

        for pt in points_to_color {
            image.put_pixel(pt.0 as u32, pt.1 as u32, Rgb(color));
        }

        let pt = move_point_one_unit(x, y, angle);
        x = pt.0;
        y = pt.1;
    }
}