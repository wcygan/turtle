use crate::algorithms::{index_to_coordinates, into_rgb, new_image_buffer, Create, RGB_CHUNK_SIZE};
use crate::arguments::Arguments;
use image::RgbImage;
use num_complex::Complex64;
use rayon::prelude::*;

pub struct MandelbrotFractal {}

impl Create for MandelbrotFractal {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image = new_image_buffer(args);

        image
            .par_chunks_mut(RGB_CHUNK_SIZE)
            .enumerate()
            .for_each(|(i, p)| {
                let (x, y) = index_to_coordinates(i as u32, args.size);
                let inner_height = args.size as f64;
                let inner_width = args.size as f64;
                let inner_y = y as f64;
                let inner_x = x as f64;

                let zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
                let zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

                let mut i = args.iterations;

                let c = Complex64::new(zx, zy);
                let mut z = c;

                while (z * z).re <= 4.0 && i > 1 {
                    z = z * z + c;
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
