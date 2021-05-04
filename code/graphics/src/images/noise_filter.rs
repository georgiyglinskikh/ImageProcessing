use crate::images::filter::Filter;
use crate::images::utils::{ImageType, PixelType};
use image::Rgba;

pub trait NoiseFilter: Filter {
    fn noise_filter(&mut self, r: u32);
}

fn noise_filterer(image: ImageType) -> PixelType {
    // TODO: WB ONLY THAN TRANSFORM INTO CLOLORED

    let average_value = {
        let mut average = [0u32; 4];
        let mut counter = 0;

        for (_, _, color) in image.enumerate_pixels() {
            average[0] += color.0[0] as u32;
            average[1] += color.0[1] as u32;
            average[2] += color.0[2] as u32;
            average[3] += color.0[3] as u32;

            counter += 1;
        }

        let mut result = [0u8; 4];

        for i in 0..average.len() {
            result[i] = (average[i] / counter) as u8;
        }

        result
    };

    Rgba::from(average_value)
}

impl NoiseFilter for ImageType {
    fn noise_filter(&mut self, r: u32) {
        Filter::filter(self, r, noise_filterer)
    }
}