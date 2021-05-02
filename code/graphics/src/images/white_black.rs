use super::ChangeColor;

use super::utils::{
    ImageType,
    white_black::WhiteBlackTypes,
};

pub trait WhiteBlack: ChangeColor {
    fn white_black(&mut self, transform_type: WhiteBlackTypes);
}

impl WhiteBlack for ImageType {
    fn white_black(&mut self, transform_type: WhiteBlackTypes){
        self.change_color(transform_type.get_value())
    }
}