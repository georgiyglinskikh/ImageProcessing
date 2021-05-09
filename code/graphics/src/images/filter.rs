use crate::images::buffer::Buffer;
use crate::images::part::Part;
use crate::images::types::{Dimension, Position, Space};
use crate::images::utils::WhiteBlackType;

pub trait Filter: Part {
    fn filter(&mut self, r: u32, func: fn(Self) -> WhiteBlackType);
}


impl Filter for Buffer {
    fn filter(&mut self, r: u32, func: fn(Self) -> WhiteBlackType) {
        let r = r as isize;
        let half_r = r / 2;

        let shift = |x: isize| x - half_r;

        let mut result = Vec::with_capacity((self.size.width * self.size.height) as usize);

        for _ in 0..self.size.width * self.size.height {
            result.push(0);
        }

        for x in 0..self.size.width {
            for y in 0..self.size.height {
                result[(x + y * self.size.width) as usize] =
                    func(self.get_part(Space {
                        position: Position {
                            x: shift(x),
                            y: shift(y),
                        },
                        size: Dimension {
                            width: r,
                            height: r,
                        },
                    }));
            }
        }

        self.buffer = result;
    }
}
