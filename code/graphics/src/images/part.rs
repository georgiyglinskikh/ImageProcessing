use crate::images::utils::ImageType;
use image::GenericImage;
use crate::images::types::{Space, Dimension, Position};

pub trait Part {
    fn get_part(&self, space: Space) -> Self;
}

impl Part for ImageType {
    fn get_part(&self, space: Space) -> Self {
        let mut result = self.clone();

        let size = Dimension {
            width: result.width() as isize,
            height: result.height() as isize,
        };

        let Space {
            position: Position {
                x,
                y
            },
            size: Dimension {
                width,
                height
            }
        } = size.fit(space);

        let result = result.sub_image(x as u32, y as u32, width as u32, height as u32);

        result.to_image()
    }
}