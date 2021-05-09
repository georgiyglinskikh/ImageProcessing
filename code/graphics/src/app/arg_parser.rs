use clap::{App, Arg, ArgMatches};

pub struct Parser<'a> {
    matches: ArgMatches<'a>,
}

impl Parser<'_> {
    pub fn new() -> Self {
        Parser {
            matches: Default::default(),
        }
    }

    pub fn parse_args(&mut self) {
        let parser = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .value_name("FILE_I")
                    .help("Input image we\'ll be processing")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE_O")
                    .help("Path to the resulting picture")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("white-black")
                    .short("wb")
                    .long("white-black")
                    .value_name("WB_TYPE")
                    .help("Make picture white and black with WB_TYPE (Smooth1/Smooth2/Flat) filter")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("noise-filter")
                    .short("n")
                    .long("noise-filter")
                    .value_name("R")
                    .help("Filter image from noise with radius R")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("sobel-filter")
                    .short("s")
                    .long("sobel-filter")
                    .value_name("R")
                    .help("Line out borders of objects in picture")
                    .takes_value(true)
            );

        self.matches = parser.get_matches()
    }

    pub fn get_value<S: AsRef<str>>(&self, name: S) -> &str {
        self.matches.value_of(name).unwrap()
    }

    pub fn is_value<S: AsRef<str>>(&self, name: S) -> bool {
        self.matches.value_of(name).is_some()
    }
}
