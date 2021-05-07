use crate::images::buffer::Buffer;
use crate::images::types::{Position, Space};
use crate::images::utils::WhiteBlackType;

pub trait Part {
    fn get_part(&self, space: Space) -> Self;
}

impl Part for Buffer {
    fn get_part(&self, space: Space) -> Self {
        let space = self.size.fit(space);

        let mut result = Vec::<WhiteBlackType>::with_capacity(
            (space.size.width * space.size.height) as usize,
        );

        let get = self.clone_buffer();

        for x in 0..space.size.width as isize {
            for y in 0..space.size.height as isize {
                result.push({
                    let position = Position {
                        x: space.position.x + x,
                        y: space.position.y + y,
                    };

                    get(position.x as usize, position.y as usize)
                });
            }
        }

        Buffer {
            size: space.size,
            buffer: result,
            image: Default::default(),
        }
    }
}