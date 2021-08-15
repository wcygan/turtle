/*
 * Turtle is a program that generates art!
 *
 * Author: Will Cygan, 2021
 */

mod algorithms;
mod arguments;
mod dispatcher;

use crate::dispatcher::dispatch;

use crate::arguments::Arguments;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Turtle")
        .version("0.1")
        .author("Will Cygan <wcygan3232@gmail.com>")
        .about("Turtle creates generative art!")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .help("Length of one side of the image, in pixels")
                .min_values(1)
                .max_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .help("The pattern of the image")
                .min_values(1)
                .max_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("rng")
                .short("r")
                .long("rng")
                .help("The seed used to initialize a pseudorandom number generator, default is 999")
                .default_value("999")
                .required(false),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("The name of the output image, e.g., <name>.png")
                .min_values(1)
                .max_values(1)
                .required(true),
        )
        .get_matches();

    /* fetch & dispatch arguments */
    dispatch(Arguments::new(&matches));
}
