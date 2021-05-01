use image::{Rgba, RgbaImage};

use super::ChangeColor;

use super::utils::{
    CoefficientsType,
    PixelType,
    white_black::WhiteBlackTypes,
};

pub trait WhiteBlack: ChangeColor {
    fn white_black(&mut self, transform_type: WhiteBlackTypes);
}

impl WhiteBlack for RgbaImage {
    fn white_black(&mut self, transform_type: WhiteBlackTypes){
        self.change_color(transform_type.get_value())
    }
}