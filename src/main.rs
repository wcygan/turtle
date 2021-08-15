/*
 * Turtle is a program that generates art!
 *
 * Author: Will Cygan, 2021
 */

mod algorithms;
mod dispatcher;
mod arguments;

use crate::dispatcher::dispatch;

use clap::{App, Arg};
use crate::arguments::Arguments;

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
                .help("Length of one side of the image")
                .min_values(1)
                .max_values(1)
                .required(true)
        )
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .help("The pattern of the image")
                .min_values(1)
                .max_values(1)
                .required(true)
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .help("The name of the output image, e.g., <name>.png")
                .min_values(1)
                .max_values(1)
                .required(true)
        )
        .get_matches();

    /* fetch & dispatch arguments */
    dispatch(Arguments::new(&matches));
}
