use crate::images::utils::{PixelType, ImageType};
use crate::images::part::Part;

pub trait Filter: Part {
    fn filter(&mut self, r: u32, func: fn(ImageType) -> PixelType);
}

impl Filter for ImageType {
    fn filter(&mut self, r: u32, func: fn(ImageType) -> PixelType) {
        let half_r = r / 2;

        let shift = |x: u32| x as i32 - half_r as i32;

        for x in 0..self.width() {
            for y in 0..self.height() {
                self.put_pixel(x, y, func(self.get_part(shift(x), shift(y), r as i32, r as i32)));
            }
        }
    }
}

