use crate::images::utils::{wrong, WhiteBlackType};

use super::change_color::ChangeColor;
use super::utils::{
    ColorFilterType,
    ImageType,
};
use crate::images::part::Part;
use crate::images::types::{Dimension, Position, Space};

pub trait WhiteBlack: ChangeColor {
    fn white_black(&mut self, transform_type: WhiteBlackTypes) -> WhiteBlackImage;
}

impl WhiteBlack for ImageType {
    fn white_black(&mut self, transform_type: WhiteBlackTypes) -> WhiteBlackImage {
        self.change_color(transform_type.get_value())
    }
}

pub struct WhiteBlackImage<'a> {
    size: Dimension,
    buf: &'a mut [WhiteBlackType],
}


impl WhiteBlackImage<'_> {
    pub fn new(buf: &mut [WhiteBlackType], size: Dimension) -> WhiteBlackImage {
        WhiteBlackImage {
            size,
            buf,
        }
    }

    fn get_index(&self, position: Position) -> usize {
        (position.x + position.y * self.size.width) as usize
    }

    pub fn get(&self, position: Position) -> Option<&WhiteBlackType> {
        self.buf.get(self.get_index(position))
    }

    pub fn set(&mut self, position: Position, value: WhiteBlackType) {
        let cell = self.buf.get_mut(self.get_index(position)).unwrap();

        *cell = value;
    }
}

impl Part for WhiteBlackImage<'_> {
    fn get_part(&self, space: Space) -> Self {
        let space = self.size.fit(space);

        let mut result = Vec::<WhiteBlackType>::with_capacity(
            (space.size.width * space.size.height) as usize
        );

        for x in 0..space.size.width as isize {
            for y in 0..space.size.height as isize {
                result.push({
                    let position = Position {
                        x: space.position.x + x,
                        y: space.position.y + y,
                    };

                    *self.get(position).unwrap()
                });
            }
        }

        let mut slice: &mut [u8] = &mut [];

        slice.clone_from_slice(result.as_slice());

        WhiteBlackImage {
            size: space.size,
            buf: slice,
        }
    }
}


pub type WhiteBlackFilterType = ColorFilterType;

pub enum WhiteBlackTypes {
    Smooth1,
    Smooth2,
    Flat,
}

const SMOOTH1: WhiteBlackFilterType = [0.2126, 0.7152, 0.0722];
const SMOOTH2: WhiteBlackFilterType = [0.299, 0.587, 0.114];
const FLAT: WhiteBlackFilterType = [0.3333, 0.3334, 0.3333];

impl WhiteBlackTypes {
    pub fn get_value(self) -> WhiteBlackFilterType {
        match self {
            WhiteBlackTypes::Smooth1 => SMOOTH1,
            WhiteBlackTypes::Smooth2 => SMOOTH2,
            WhiteBlackTypes::Flat => FLAT,
        }
    }

    pub fn from_string(name: &str) -> Option<WhiteBlackTypes> {
        match name {
            "Smooth1" => Some(WhiteBlackTypes::Smooth1),
            "Smooth2" => Some(WhiteBlackTypes::Smooth2),
            "Flat" => Some(WhiteBlackTypes::Flat),
            _ => {
                wrong();
                None
            }
        }
    }
}