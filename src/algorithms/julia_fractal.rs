use image::RgbImage;
use rayon::prelude::*;

use crate::algorithms::{index_to_coordinates, into_rgb, new_image_buffer, Create, RGB_CHUNK_SIZE};
use crate::arguments::Arguments;
use num_complex::Complex64;

pub struct JuliaFractal {}

impl Create for JuliaFractal {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);

        image
            .par_chunks_mut(RGB_CHUNK_SIZE)
            .enumerate()
            .for_each(|(i, p)| {
                let (x, y) = index_to_coordinates(i as u32, args.size);
                let inner_height = args.size as f32;
                let inner_width = args.size as f32;
                let inner_y = y as f32;
                let inner_x = x as f32;

                let mut zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
                let mut zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

                let mut i = args.iterations;

                while zx * zx + zy * zy < 4.0 && i > 1 {
                    let tmp = zx * zx - zy * zy + args.cx;
                    zy = 2.0 * zx * zy + args.cy;
                    zx = tmp;
                    i -= 1;
                }

                let r = (i << 3) as u8;
                let g = (i << 5) as u8;
                let b = (i * 4) as u8;
                let pixel = into_rgb(r, g, b);
                p.copy_from_slice(&pixel);
            });

        image
    }
}

impl JuliaFractal {
    pub fn weird(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);
        let c = Complex64::new(args.cx as f64, args.cy as f64);

        image
            .par_chunks_mut(RGB_CHUNK_SIZE)
            .enumerate()
            .for_each(|(i, p)| {
                let (x, y) = index_to_coordinates(i as u32, args.size);
                let inner_height = args.size as f64;
                let inner_width = args.size as f64;
                let inner_y = y as f64;
                let inner_x = x as f64;

                let mut zx = 2.0 * (inner_x - 0.7 * inner_width) / (inner_width * 1.4);
                let mut zy = 1.3 * (inner_y - 0.3 * inner_height) / (inner_height * 1.4);

                let mut i = args.iterations;

                let mut z = Complex64::new(zx, zy);
                while (z + z).re <= 4.0 && i > 1 {
                    z = z * z + c;
                    i -= 1;
                }

                let r = (i << 4) as u8;
                let g = (i << 6) as u8;
                let b = (i * 3) as u8;
                let pixel = into_rgb(r, g, b);
                p.copy_from_slice(&pixel);
            });

        image
    }
}
