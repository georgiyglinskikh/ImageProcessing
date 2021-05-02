use super::utils::{ColorFilterType, PixelType, ImageType};

pub trait ChangeColor {
    fn change_color(&mut self, coefficient: ColorFilterType);
}

impl ChangeColor for ImageType {
    fn change_color(&mut self, coefficient: ColorFilterType) {
        for (_, _, color) in self.enumerate_pixels_mut() {
            *color = PixelType::from({
                let value =
                    color.0.iter().take(3)
                        .zip(coefficient.iter())
                        .map(|x| ((*x.0 as f64) * *x.1) as u8)
                        .sum();

                let alpha = *color.0.iter().last().unwrap();

                [value, value, value, alpha]
            })
        }
    }
}