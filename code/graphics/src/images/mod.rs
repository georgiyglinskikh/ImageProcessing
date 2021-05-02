pub mod white_black;
pub mod utils;

use utils::ImageType;
use crate::images::utils::{ColorFilterType, PixelType};

pub trait ChangeColor {
    fn change_color(&mut self, coefficient: ColorFilterType);
}

impl ChangeColor for ImageType {
    fn change_color(&mut self, coefficient: ColorFilterType) {
        for (_, _, pixel) in self.enumerate_pixels_mut() {
            *pixel = PixelType::from({
                let value =
                    pixel.0.iter().take(3)
                        .zip(coefficient.iter())
                        .map(|x| ((*x.0 as f64) * *x.1) as u8)
                        .sum();

                let alpha = pixel.0[pixel.0.len() - 1];

                [value, value, value, alpha]
            })
        }
    }
}
