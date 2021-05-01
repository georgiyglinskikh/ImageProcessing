use image::Rgba;
use std::ops::Mul;

pub type PixelType = Rgba<u8>;
pub type CoefficientsType = [f32; 4];

pub mod white_black {
    pub enum WhiteBlackTypes {
        Smooth1,
        Smooth2,
        Flat,
    }

    const SMOOTH1: super::CoefficientsType = [0.2126, 0.7152, 0.0722, 1.0];
    const SMOOTH2: super::CoefficientsType = [0.299, 0.587, 0.114, 1.0];
    const FLAT: super::CoefficientsType = [0.3333, 0.3334, 0.3333, 1.0];

    impl WhiteBlackTypes {
        pub fn get_value(self) -> super::CoefficientsType {
            match self {
                WhiteBlackTypes::Smooth1 => SMOOTH1,
                WhiteBlackTypes::Smooth2 => SMOOTH2,
                WhiteBlackTypes::Flat => FLAT,
            }
        }
    }
}
