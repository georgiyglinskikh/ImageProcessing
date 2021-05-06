use crate::images::utils::{ImageType, WhiteBlackType};
use image::GenericImage;
use crate::images::types::{Space, Dimension, Position};
use crate::images::white_black::WhiteBlackImage;

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

impl Part for WhiteBlackImage<'_> {
    fn get_part(&self, space: Space) -> Self {
        let space = self.size.fit(space);

        let mut result = Vec::<WhiteBlackType>::with_capacity(
            (space.size.width * space.size.height) as usize + 1
        );

        for x in 0..space.size.width as isize {
            for y in 0..space.size.height as isize {
                result.push({
                    let position = Position {
                        x: space.position.x + x,
                        y: space.position.y + y,
                    };

                    *self.get(position).unwrap()
                });
            }
        }

        let mut slice: &mut [u8] = &mut [];

        slice.clone_from_slice(result.as_slice());

        WhiteBlackImage {
            size: space.size,
            buf: slice,
        }
    }
}