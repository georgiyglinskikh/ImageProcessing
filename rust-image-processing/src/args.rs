use std::{borrow::Borrow, path::Path};

use clap::{App, Arg, ArgMatches};

pub struct ImageArgs<'a> {
    width: usize,
    height: usize,
    path: &'a Path,
}

impl ImageArgs<'_> {
    pub fn new<'a>(width: usize, height: usize, path: &'a Path) -> ImageArgs<'a> {
        ImageArgs {
            width,
            height,
            path,
        }
    }

    pub fn parse_args<'a>() -> ImageArgs<'a> {
        let matches: &'a ArgMatches<'a> = App::new("rust-image-processing")
            .version("0.1.0")
            .author("georgiyglinskikh <georgiyglinskikh@gmail.com>")
            .about("Defines camera`s moving direction by two images")
            .arg(
                Arg::with_name("path")
                    .short("p")
                    .long("file")
                    .takes_value(true)
                    .help("Path to file: full"),
            )
            .arg(
                Arg::with_name("size")
                    .short("s")
                    .long("size")
                    .takes_value(true)
                    .help("Image size: WxH"),
            )
            .get_matches()
            .borrow();

        let size: Vec<usize> = matches
            .value_of("size")
            .expect("You should enter size of image (see --help)")
            .split_terminator("x")
            .take(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let path: &'a Path = Path::new(
            matches
                .value_of("path")
                .expect("You should enter path to image (see --help)")
                .clone(),
        );

        ImageArgs::new(size[0], size[1], path)
    }
}
