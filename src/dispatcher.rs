
use crate::arguments::Arguments;
use crate::algorithms::{*, self};

pub fn dispatch(args: Arguments)  {
    match args.pattern.as_str() {
        "square" => {
            algorithms::square::Square::create(args);
        }
        "circle" => {
            todo!("Circle isn't implemented yet!");
        }
        _ => {
            eprintln!("Please provide a valid pattern, e.g.,");
            eprintln!("\t'square'");
            eprintln!("\t'circle'");
            eprintln!("\nThank you! Goodbye...");
            std::process::exit(1);
        }
    }
}