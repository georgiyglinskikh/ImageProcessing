use crate::images::buffer::Buffer;
use crate::images::utils::{ColorType, WhiteBlackType, PixelType};

use super::utils::ColorFilterType;

pub trait ChangeColor {
    fn change_color(&mut self, color_filter: ColorFilterType);
}

pub trait MaskColor {
    fn mask_color(&mut self);
}

impl ChangeColor for Buffer {
    fn change_color(&mut self, color_filter: ColorFilterType) {
        for (_, _, color) in self.image.enumerate_pixels() {
            let value = get_value(color.0, color_filter).iter().sum();

            self.buffer.push(value);
        }
    }
}

impl MaskColor for Buffer {
    fn mask_color(&mut self) {
        let max = *self.buffer.iter().max().unwrap();

        let get = self.clone_buffer();

        for (x, y, color) in self.image.enumerate_pixels_mut() {
            let value = get(x as usize, y as usize);

            let coefficient = value as f64 / max as f64;

            let res = get_value(color.0, [coefficient, coefficient, coefficient]);

            *color = PixelType::from([res[0], res[1], res[2], color.0[3]]);
        }
    }
}

fn get_value(color: ColorType, filter: ColorFilterType) -> Vec<WhiteBlackType> {
    color
        .iter()
        .take(3)
        .zip(filter.iter())
        .map(|x| ((*x.0 as f64) * *x.1) as WhiteBlackType)
        .collect()
}
