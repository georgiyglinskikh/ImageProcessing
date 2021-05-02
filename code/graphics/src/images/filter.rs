use crate::images::utils::{PixelType, ImageType};
use image::{EncodableLayout, GenericImage};

trait Filter {
    fn filter(&mut self, r: u8, func: fn(&[PixelType]) -> PixelType);
}

impl Filter for ImageType {
    fn filter(&mut self, r: u8, func: fn(&[PixelType]) -> PixelType) {
        let old_image = self.clone().as_bytes();

        // TODO: Add logic
    }
}

trait Part {
    fn get_part(&self, x: u32, y: u32, width: u32, height: u32) -> Self;
}

impl Part for ImageType {
    fn get_part(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let mut result = self.clone();

        result.sub_image(x, y, width, height);

        result
    }
}