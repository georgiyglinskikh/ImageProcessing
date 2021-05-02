use image::{Rgba, RgbaImage};

pub type ImageType = RgbaImage;
pub type PixelType = Rgba<u8>;
pub type ColorFilterType = [f64; 3];

pub fn wrong() {
    panic!("You should choose right option - check \'--help\'")
}
