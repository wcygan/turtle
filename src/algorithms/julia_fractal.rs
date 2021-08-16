use crate::algorithms::Create;
use crate::arguments::Arguments;
use image::{ImageBuffer, Rgb, RgbImage};

pub struct JuliaFractal {}

impl Create for JuliaFractal {
    fn create(args: &mut Arguments) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(args.size as u32, args.size as u32);

        for x in 0..args.size {
            for y in 0..args.size {
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
                let pixel = Rgb([r, g, b]);
                image.put_pixel(x as u32, y as u32, pixel);
            }
        }

        image
    }
}
