use crate::images::utils::wrong;

use super::change_color::ChangeColor;
use super::utils::{
    ColorFilterType,
    ImageType,
};

pub trait WhiteBlack: ChangeColor {
    fn white_black(&mut self, transform_type: WhiteBlackTypes);
}

impl WhiteBlack for ImageType {
    fn white_black(&mut self, transform_type: WhiteBlackTypes) {
        self.change_color(transform_type.get_value())
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
            _ => { wrong(); None }
        }
    }
}