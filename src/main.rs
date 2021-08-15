/*
 * Turtle is a program that generates art
 *
 * Author: Will Cygan, 2021
 */

use clap::{App, Arg};

// TODO: Use crate "image" to output files
// TODO: Look for parts of code that could be parallelized with rayon
fn main() {
    let matches = App::new("Turtle")
        .version("0.1")
        .author("Will Cygan <wcygan3232@gmail.com>")
        .about("Turtle creates generative art!")
        .arg(
            Arg::with_name("size")
                .short("s")
                .help(
                    "Length of one side of the image",
                )
                .min_values(1)
                .max_values(1)
                .required(true)
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .help(
                    "The name of the output image, e.g., <name>.png",
                )
                .min_values(1)
                .max_values(1)
                .required(true)
        )
        .get_matches();

    let size = matches.value_of("size").unwrap().parse::<u32>().unwrap();
    let name = matches.value_of("name").unwrap();
}
