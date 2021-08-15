use clap::ArgMatches;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct Arguments {
    pub size: u64,
    pub name: String,
    pub pattern: String,
    pub rng: Box<StdRng>,
}

impl Arguments {
    pub fn new(matches: &ArgMatches) -> Arguments {
        Arguments {
            size: matches
                .value_of("size")
                .unwrap()
                .parse::<u64>()
                .expect("size must be a positive integer"),
            name: matches.value_of("name").unwrap().to_string(),
            pattern: matches.value_of("pattern").unwrap().to_string(),
            rng: Box::new(StdRng::seed_from_u64(
                matches
                    .value_of("rng")
                    .unwrap()
                    .parse::<u64>()
                    .expect("rng must be a positive integer"),
            )),
        }
    }
}
