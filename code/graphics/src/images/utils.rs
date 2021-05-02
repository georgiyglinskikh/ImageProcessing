use image::{Rgba, RgbaImage};

pub type ImageType = RgbaImage;
pub type PixelType = Rgba<u8>;
pub type ColorFilterType = [f64; 3];

pub mod white_black {
    pub type WhiteBlackFilterType = super::ColorFilterType;

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
    }
}
