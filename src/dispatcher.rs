use crate::arguments::Arguments;

pub fn dispatch(args: Arguments)  {
    match args.pattern.as_str() {
        "square" => {
            println!("Got square!")
            // TODO: implement square
        }
        "circle" => {
            println!("Got circle!")
            // TODO: implement circle
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