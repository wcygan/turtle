use image::RgbImage;

use crate::algorithms::{self, *};
use crate::arguments::Arguments;

pub fn dispatch(mut args: Arguments) {
    let img: RgbImage = match args.pattern.as_str() {
        "square" => algorithms::square::Square::create(&mut args),
        "circle" => algorithms::circle::Circle::create(&mut args),
        "julia-fractal" => algorithms::julia_fractal::JuliaFractal::create(&mut args),
        _ => {
            eprintln!("Please provide a valid pattern, e.g.,");
            eprintln!("\t'-p square'");
            eprintln!("\t'-p circle'");
            eprintln!("\t'-p julia-fractal'");
            eprintln!("\nThank you! Goodbye...");
            std::process::exit(1);
        }
    };

    img.save(format!("{}.png", args.name))
        .expect("The image failed to save...");
}
