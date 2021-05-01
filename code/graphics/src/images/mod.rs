use image::{Rgba, RgbaImage};
use test::bench::iter;

pub mod white_black;
pub mod utils;

pub trait ChangeColor {
    fn change_color(&mut self, coefficient: utils::CoefficientsType);
}

impl ChangeColor for RgbaImage {
    fn change_color(&mut self, coefficient: utils::CoefficientsType) {
        for (_, _, pixel) in self.enumerate_pixels_mut() {
            *pixel = Rgba::<u8>::from({

            })
        }
    }
}
