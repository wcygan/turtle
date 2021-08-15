use crate::algorithms::{self, *};
use crate::arguments::Arguments;
use image::RgbImage;

pub fn dispatch(mut args: Arguments) {
    let img: RgbImage = match args.pattern.as_str() {
        "square" => algorithms::square::Square::create(&mut args),
        "circle" => algorithms::circle::Circle::create(&mut args),
        "fractal" => {
            // https://github.com/image-rs/image/blob/master/README.md#generating-fractals
            todo!("Fractal isn't implemented yet!")
        }
        _ => {
            eprintln!("Please provide a valid pattern, e.g.,");
            eprintln!("\t'square'");
            eprintln!("\t'circle'");
            eprintln!("\nThank you! Goodbye...");
            std::process::exit(1);
        }
    };

    img.save(format!("{}.png", args.name))
        .expect("The image failed to save...");
}
