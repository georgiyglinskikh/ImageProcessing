use crate::images::utils::{PixelType, ImageType};
use image::EncodableLayout;

trait Filter {
    fn filter(&mut self, r: u8, func: fn(&[PixelType]) -> PixelType);
}

impl Filter for ImageType {
    fn filter(&mut self, r: u8, func: fn(&[PixelType]) -> PixelType) {
        let old_image = self.clone().as_bytes();


    }
}

trait Part {
    fn get_part(&self, x: isize, y: isize, width: isize, height: isize) -> &[PixelType];
}

impl Part for ImageType {
    fn get_part(&self, x: isize, y: isize, width: isize, height: isize) -> &[PixelType] {
        self.
    }
}