use image::{Rgba, RgbaImage};

pub type ImageType = RgbaImage;
pub type PixelType = Rgba<WhiteBlackType>;
pub type CoefficientType = f64;
pub type ColorFilterType = [CoefficientType; 3];
pub type AlphaFilterType = CoefficientType;
pub type ColorType = [WhiteBlackType; 4];
pub type ChannelsType = [WhiteBlackType; 3];

pub type WhiteBlackType = u8;

pub fn wrong() {
    panic!("You should choose right option - check \'--help\'")
}
