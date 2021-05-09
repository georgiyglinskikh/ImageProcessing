use crate::images::buffer::Buffer;
use crate::images::change_color::MaskColor;
use crate::images::noise_filter::NoiseFilter;
use crate::images::types::Dimension;
use crate::images::white_black::WhiteBlack;
use crate::images::white_black::WhiteBlackTypes;
use crate::{app::arg_parser::Parser, images::sobel::Sobel};

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
        let mode = if self.parser.is_value("white-black") {
            let mode_str = self.parser.get_value("white-black");

            WhiteBlackTypes::from_string(mode_str).unwrap()
        } else {
            WhiteBlackTypes::Smooth1
        };

        self.buffer.white_black(mode);

        if self.parser.is_value("noise-filter") {
            let r: u32 = self
                .parser
                .get_value("noise-filter")
                .parse()
                .expect("Cannot interpret R as number");

            self.buffer.noise_filter(r);
        }

        if self.parser.is_value("sobel-filter") {
            let r: u32 = self
                .parser
                .get_value("sobel-filter")
                .parse()
                .expect("Cannot interpret R as number");

            self.buffer.sobel(r);
        }
    }

    pub fn save_image(&mut self) {
        if self.parser.is_value("white-black") {
            self.buffer.update_image();
        } else if self.parser.is_value("noise-filter") || self.parser.is_value("sobel-filter") {
            self.buffer.mask_color();
        }

        self.buffer
            .image
            .save(self.parser.get_value("output"))
            .expect("Cannot save image to this file");
    }
}
