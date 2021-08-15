use clap::ArgMatches;

pub struct Arguments {
    pub size: u32,
    pub name: String,
    pub pattern: String,
}

impl Arguments {
    pub fn new(matches: &ArgMatches) -> Arguments {
        Arguments {
            size: matches.value_of("size").unwrap().parse::<u32>().expect("size must be a positive integer"),
            name: matches.value_of("name").unwrap().to_string(),
            pattern: matches.value_of("pattern").unwrap().to_string()
        }
    }
}