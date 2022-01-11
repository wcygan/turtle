use image::RgbImage;

use crate::algorithms::{self, *};
use crate::arguments::Arguments;

pub fn dispatch(mut args: Arguments) {
    let img: RgbImage = match args.pattern.as_str() {
        "diamond" => algorithms::diamond::Diamond::create(&mut args),
        "snowflake" => algorithms::snowflake::Snowflake::create(&mut args),
        "tree" => algorithms::tree::Tree::create(&mut args),
        "airbrush" => algorithms::airbrush::Airbrush::create(&mut args),
        "squiggly" => algorithms::squiggly::Squiggly::create(&mut args),
        "pixels" => algorithms::pixels::Pixels::create(&mut args),
        "square" => algorithms::square::Square::create(&mut args),
        "circle" => algorithms::circle::Circle::create(&mut args),
        "julia-fractal" => algorithms::julia_fractal::JuliaFractal::create(&mut args),
        "julia-weird" => algorithms::julia_fractal::JuliaFractal::weird(&mut args),
        "blobs" => algorithms::blobs::Blobs::create(&mut args),
        "mandelbrot-fractal" => {
            algorithms::mandelbrot_fractal::MandelbrotFractal::create(&mut args)
        }
        _ => {
            eprintln!("Please provide a valid pattern, e.g.,");
            eprintln!("\t'-p diamond'");
            eprintln!("\t'-p snowflake'");
            eprintln!("\t'-p tree'");
            eprintln!("\t'-p airbrush'");
            eprintln!("\t'-p squiggly'");
            eprintln!("\t'-p julia-fractal'");
            eprintln!("\t'-p julia-weird'");
            eprintln!("\t'-p mandelbrot-fractal'");
            eprintln!("\t'-p blobs'");
            eprintln!("\t'-p pixels'");
            eprintln!("\t'-p square'");
            eprintln!("\t'-p circle'");
            eprintln!("\nThank you! Goodbye...");
            std::process::exit(1);
        }
    };

    img.save(format!("{}.png", args.name))
        .expect("The image failed to save...");
}
