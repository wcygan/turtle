use std::ops::Add;

use cgmath::{Angle, Deg, Rad, Vector2};
use image::RgbImage;
use rand::{Rng, RngCore, thread_rng};
use rand::prelude::ThreadRng;

use crate::algorithms::{convert_if_out_of_bounds, Create, new_image_buffer, point_is_in_rectangle, random_angle, random_color, random_points, xy_within_radius_from_center};
use crate::arguments::Arguments;

pub struct Squiggly {}

impl Create for Squiggly {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let mut rng = rand::thread_rng();

        let points = random_points(args, 50);
        for (x, y) in points {
            let color = random_color(args);
            draw_squiggly_line(&mut image, x, y, color, &mut rng)
        }

        todo!("Not implemented: Squiggly");
        image
    }
}

// use: is_valid_point(x, y, center_x, center_y, radius)

pub fn draw_squiggly_line(
    image: &mut RgbImage,
    x: u32,
    y: u32,
    color: [u8; 3],
    rng: &mut ThreadRng,
) {
    let (w, h) = image.dimensions();
    let initial_angle = random_angle(rng);

    while point_is_in_rectangle(x, y, w, h) {}
}