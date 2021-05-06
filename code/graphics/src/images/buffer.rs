use crate::images::types::{Dimension, Position};
use crate::images::utils::{WhiteBlackType, ImageType};
use core::slice::SlicePattern;

pub struct Buffer<'buf> {
    pub size: Dimension,
    pub buf: &'buf mut [WhiteBlackType],
    pub image: ImageType
}

impl Buffer<'_> {
    pub fn new(image: ImageType) -> Buffer {
        Buffer {
            size: Dimension {
                width: image.width() as isize,
                height: image.height() as isize
            },
            buf: image.as_(),
            image
        }
    }

    fn get_index(&self, position: Position) -> usize {
        (position.x + position.y * self.size.width) as usize
    }

    pub fn get(&self, position: Position) -> Option<&WhiteBlackType> {
        self.buf.get(self.get_index(position))
    }

    pub fn set(&mut self, position: Position, value: WhiteBlackType) {
        let cell = self.buf.get_mut(self.get_index(position)).unwrap();

        *cell = value;
    }

    pub fn to_image(&self, image: ImageType) -> ImageType {
        let mut res = image;

        for (x, y, color) in res.enumerate_pixels_mut() {
            let value = *self.get(Position {x: x as isize, y: y as isize }).unwrap();

            *color = PixelType::from([value, value, value, color.0[4]]);
        }

        res
    }
}