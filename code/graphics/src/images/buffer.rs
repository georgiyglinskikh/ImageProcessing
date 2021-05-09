use crate::images::types::Dimension;
use crate::images::utils::{BufferType, ImageType, PixelType};

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

    pub fn update_image(&mut self) {
        for (x, y, color) in self.image.enumerate_pixels_mut() {
            let value = self.buffer[(x + y * self.size.width as u32) as usize];

            *color = PixelType::from([value, value, value, color.0[3]]);
        }
    }
}
