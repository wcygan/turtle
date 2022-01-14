/*
 * Turtle is a program that generates art!
 *
 * Author: Will Cygan, 2021
 */

use clap::{App, Arg};

use crate::arguments::Arguments;
use crate::dispatcher::dispatch;

mod algorithms;
mod arguments;
mod dispatcher;

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
                .min_values(0)
                .max_values(1)
                .required(false),
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
                .help("The seed used to initialize a pseudorandom number generator")
                .default_value("999")
                .required(false),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("The name of the output image, e.g., <name>.png")
                .min_values(0)
                .max_values(1)
                .required(false),
        )
        .arg(
            Arg::with_name("x")
                .allow_hyphen_values(true)
                .short("x")
                .long("complex-imaginary")
                .help("The imaginary part of a complex number")
                .default_value("-0.55")
                .required(false),
        )
        .arg(
            Arg::with_name("y")
                .short("y")
                .long("complex-real")
                .help("The real part of a complex number")
                .default_value("0.53")
                .required(false),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .help("The number of times to executed iterated algorithms")
                .default_value("105")
                .required(false),
        )
        .get_matches();

    /* fetch & dispatch arguments */
    dispatch(Arguments::new(&matches));
}
