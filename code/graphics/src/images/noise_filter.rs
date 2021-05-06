use crate::images::filter::Filter;
use crate::images::utils::{ImageType, PixelType, WhiteBlackType};
use image::Rgba;
use crate::images::white_black::WhiteBlackImage;

pub trait NoiseFilter: Filter {
    fn noise_filter(&mut self, r: u32);
}

fn noise_filterer(image: WhiteBlackImage) -> WhiteBlackType {
    (image.buf.iter().sum::<isize>() / image.size.width * image.size.height) as u8
}

impl NoiseFilter for WhiteBlackImage {
    fn noise_filter(&mut self, r: u32) {
        Filter::filter(self, r, noise_filterer)
    }
}