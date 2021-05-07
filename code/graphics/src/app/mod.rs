use crate::app::arg_parser::Parser;
use crate::images::buffer::Buffer;
use crate::images::change_color::MaskColor;
use crate::images::noise_filter::NoiseFilter;
use crate::images::types::Dimension;
use crate::images::white_black::WhiteBlack;
use crate::images::white_black::WhiteBlackTypes;

mod arg_parser;

pub struct App<'a> {
    buffer: Buffer,
    parser: Parser<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            buffer: Buffer {
                image: Default::default(),
                buffer: Default::default(),
                size: Dimension {
                    width: 0,
                    height: 0,
                },
            },
            parser: Parser::new(),
        }
    }

    pub fn parse_args(&mut self) {
        self.parser.parse_args()
    }

    pub fn open_image(&mut self) {
        let image = image::open(self.parser.get_value("input")).expect("Cannot open image");

        self.buffer = Buffer::new(image.to_rgba8());
    }

    pub fn process_image(&mut self) {
        if self.parser.is_value("white-black") {
            let mode_str = self.parser.get_value("white-black");

            let mode = WhiteBlackTypes::from_string(mode_str).unwrap();

            self.buffer.white_black(mode);

            self.buffer.update_image();
        } else if self.parser.is_value("filter") {
            let r: u32 = self
                .parser
                .get_value("filter")
                .parse()
                .expect("Cannot interpret R as number");

            self.buffer.white_black(WhiteBlackTypes::Smooth1);

            self.buffer.noise_filter(r);

            self.buffer.mask_color();
        }
    }

    pub fn save_image(&mut self) {
        self.buffer
            .image
            .save(self.parser.get_value("output"))
            .expect("Cannot save image to this file");
    }
}
