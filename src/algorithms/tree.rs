
use std::ops::{Add};

use cgmath::{Angle, Deg};
use image::{Rgb, RgbImage};
use rand::{thread_rng};


use crate::algorithms::{angle_between_points_mirrored_y, Create, move_point_one_unit, new_image_buffer, point_is_in_rectangle, random_color, random_edge_point};
use crate::arguments::Arguments;

pub struct Tree {}

impl Create for Tree {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let (x, y) = random_edge_point(args);
        draw_tree(args, &mut image, x, y);
        image
    }
}

fn draw_tree(
    args: &mut Arguments,
    image: &mut RgbImage,
    edge_x: u32,
    edge_y: u32,
) {
    let (w, h) = image.dimensions();
    let _rng = thread_rng();
    let mut angle = angle_between_points_mirrored_y(edge_x, edge_y, w / 2, h / 2);
    {
        let (i, j) = move_point_one_unit(edge_x as f64, edge_y as f64, angle);
        if !point_is_in_rectangle(i as i32, j as i32, w, h) {
            angle = angle.opposite();
        }
    }

    let steps = 4;
    let color = random_color(args);
    let mut line_length = args.size - (args.size / 4);
    let mut lines_to_draw = 1;

    let mut current_points = vec![];
    let mut next_points = vec![];
    current_points.push((edge_x, edge_y));
    next_points.push((edge_x, edge_y));


    for _step in 0..steps {
        // draw lines
        for pt in &current_points {
            let (x, y) = *pt;

            let angle_to_add = 240 / lines_to_draw;

            for i in 1..(lines_to_draw / 2) + 1 {
                let new_angle = angle.add(Deg((angle_to_add * i) as f64));
                let new_pt = draw_straight_line(image, x as f64, y as f64, w, h, line_length, color, new_angle);
                // let new_pt = move_point_n_units(pt.0 as f64, pt.1 as f64, line_length as f64, new_angle);
                next_points.push((new_pt.0, new_pt.1))
            }

            for i in 1..(lines_to_draw / 2) + 1 {
                let new_angle = angle.add(Deg((angle_to_add * i) as f64));
                let new_pt = draw_straight_line(image, x as f64, y as f64, w, h, line_length, color, new_angle);
                // let new_pt = move_point_n_units(pt.0 as f64, pt.1 as f64, line_length as f64, new_angle);
                next_points.push((new_pt.0, new_pt.1))
            }
        }

        // swap the points
        current_points.clear();
        for pt in &next_points {
            current_points.push(*pt);
        }
        next_points.clear();

        // calculate vars for the next iteration
        line_length = (0.69 * line_length as f64) as u32;
        lines_to_draw += 7;
    }
}

fn draw_straight_line(
    image: &mut RgbImage,
    mut x: f64,
    mut y: f64,
    w: u32,
    h: u32,
    length: u32,
    color: [u8; 3],
    angle: Deg<f64>,
) -> (u32, u32) {
    for _i in 0..length {
        let (rx, ry) = (x.round() as i32, y.round() as i32);
        if point_is_in_rectangle(rx, ry, w, h) {
            image.put_pixel(rx as u32, ry as u32, Rgb(color));
        }
        let pt = move_point_one_unit(x, y, angle);
        x = pt.0;
        y = pt.1;
    }

    (x.round() as u32, y.round() as u32)
}