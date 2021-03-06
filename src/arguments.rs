use clap::ArgMatches;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct Arguments {
    pub size: u32,
    pub name: String,
    pub pattern: String,
    pub rng: Box<StdRng>,
    pub cx: f32,
    pub cy: f32,
    pub iterations: u32,
}

impl Arguments {
    pub fn new(matches: &ArgMatches) -> Arguments {
        let pattern = matches.value_of("pattern").unwrap();
        let name = matches.value_of("name").unwrap_or(pattern);
        let size = matches
            .value_of("size")
            .unwrap_or("1000")
            .parse::<u32>().unwrap();
        Arguments {
            size,
            name: name.to_string(),
            pattern: pattern.to_string(),
            rng: Box::new(StdRng::seed_from_u64(
                matches
                    .value_of("rng")
                    .unwrap()
                    .parse::<u64>()
                    .expect("rng must be a positive integer"),
            )),
            cx: matches
                .value_of("x")
                .unwrap()
                .parse::<f32>()
                .expect("complex-imaginary must be a float"),
            cy: matches
                .value_of("y")
                .unwrap()
                .parse::<f32>()
                .expect("complex-imaginary must be a float"),
            iterations: matches
                .value_of("iterations")
                .unwrap()
                .parse::<u32>()
                .expect("iterations must be a positive integer"),
        }
    }
}
