use std::ops::IndexMut;

use crate::images::types::{Dimension, Position};
use crate::images::utils::{
    BufferType, ImageType, PixelType,
    WhiteBlackType,
};

pub struct Buffer {
    pub size: Dimension,
    pub buffer: BufferType,
    pub image: ImageType,
}

impl Buffer {
    pub fn new(image: ImageType) -> Buffer {
        let width = image.width() as isize;
        let height = image.height() as isize;

        Buffer {
            size: Dimension { width, height },
            buffer: vec![],
            image,
        }
    }

    pub fn set(&mut self, position: Position, value: WhiteBlackType) {
        *self
            .buffer
            .index_mut((position.x + position.y * self.size.width) as usize) = value;
    }

    pub fn update_image(&mut self) {
        let get = self.clone_buffer();

        for (x, y, color) in self.image.enumerate_pixels_mut() {
            let value = get(x as usize, y as usize);

            *color = PixelType::from([value, value, value, color.0[3]]);
        }
    }

    pub fn clone_buffer(&self) -> impl Fn(usize, usize) -> WhiteBlackType {
        let buffer = self.buffer.clone();

        let width = self.size.width;

        move |x, y| *buffer.get(x + y * width as usize).unwrap()
    }
}
