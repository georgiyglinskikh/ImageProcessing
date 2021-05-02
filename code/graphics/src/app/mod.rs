use crate::app::arg_parser::Parser;
use crate::images::utils::ImageType;
use crate::images::white_black::WhiteBlack;
use crate::images::white_black::WhiteBlackTypes;

mod arg_parser;

pub struct App<'a> {
    buf: ImageType,
    parser: Parser<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            buf: Default::default(),
            parser: Parser::new(),
        }
    }

    pub fn parse_args(&mut self) {
        self.parser.parse_args()
    }

    pub fn open_image(&mut self) {
        let image =
            image::open(self.parser.get_value("input"))
                .expect("Cannot open image");

        self.buf = image.to_rgba8();
    }

    pub fn process_image(&mut self) {
        if self.parser.is_value("white-black") {
            let mode_str = self.parser.get_value("white-black");

            let mode = WhiteBlackTypes::from_string(mode_str).unwrap();

            self.buf.white_black(mode);
        }
    }

    pub fn save_image(&mut self) {
        self.buf.save(self.parser.get_value("output"))
            .expect("Cannot save image to this file");
    }
}