use super::utils::{ColorFilterType, PixelType, ImageType};
use crate::images::utils::{AlphaFilterType, WhiteBlackType, ColorType, ChannelsType};
use crate::images::white_black::WhiteBlackImage;
use crate::images::types::Position;

pub trait ChangeColor {
    fn change_color(&mut self, color_filter: ColorFilterType, alpha_filter: AlphaFilterType);
}

pub trait MaskColor {
    fn mask_color(self, image: ImageType) -> ImageType;
}

fn get_value(color: ColorType, filter: ColorFilterType) -> ChannelsType {
    color.iter()
        .take(3)
        .zip(filter.iter())
        .map(|x| ((*x.0 as f64) * *x.1) as WhiteBlackType)
        .collect()
}

impl ChangeColor for ImageType {
    fn change_color(&mut self, color_filter: ColorFilterType, alpha_filter: AlphaFilterType) {
        for (_, _, pixel) in self.enumerate_pixels_mut() {
            *pixel = PixelType::from({
                let value = get_value(pixel.0, color_filter).iter().sum();

                let alpha = (*pixel.0.iter().last().unwrap() * alpha_filter[0]) as WhiteBlackType;

                [value, value, value, alpha]
            })
        }
    }
}

impl MaskColor for WhiteBlackImage {
    fn mask_color(self, image: ImageType) -> ImageType {
        let mut result = image;

        let max = *self.buf.iter().max().unwrap();

        for (x, y, color) in result.enumerate_pixels_mut() {
            let value = self.get(
                Position {
                    x: x as isize,
                    y: y as isize
                }).unwrap();

            let coefficient = *value as f64 / max as f64;

            let res = get_value(color.0, [coefficient, coefficient, coefficient]);

            *color = PixelType::from([res[0], res[1], res[2], color.0[4]]);
        }

        result
    }
}