use super::utils::{ColorFilterType, PixelType, ImageType};
use crate::images::utils::{AlphaFilterType, WhiteBlackType, ColorType};

pub trait ChangeColor {
    fn change_color(&mut self, color_filter: ColorFilterType, alpha_filter: AlphaFilterType);
}

// TODO
fn get_value(color: ColorType, color_filter: color_filter) {

}

impl ChangeColor for ImageType {
    fn change_color(&mut self, color_filter: ColorFilterType, alpha_filter: AlphaFilterType) {
        for (_, _, pixel) in self.enumerate_pixels_mut() {
            *pixel = PixelType::from({
                let value =
                    pixel.0.iter().take(3)
                        .zip(color.iter())
                        .map(|x| ((*x.0 as f64) * *x.1) as WhiteBlackType)
                        .sum();

                let alpha = (*pixel.0.iter().last().unwrap() * alpha_filter[0]) as WhiteBlackType;

                [value, value, value, alpha]
            })
        }
    }


}