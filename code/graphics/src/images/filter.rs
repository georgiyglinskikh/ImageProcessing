use crate::images::utils::{PixelType, ImageType, WhiteBlackType};
use crate::images::part::Part;
use crate::images::types::{Space, Position, Dimension};
use crate::images::white_black::WhiteBlackImage;

pub trait Filter: Part {
    fn filter(&mut self, r: u32, func: fn(WhiteBlackImage) -> WhiteBlackType);
}

impl Filter for WhiteBlackImage {
    fn filter(&mut self, r: u32, func: fn(WhiteBlackImage) -> WhiteBlackType) {
        let r = r as isize;
        let half_r = r / 2;

        let shift = |x: isize| x - half_r;

        for x in 0..self.size.width {
            for y in 0..self.size.height {
                self.set(
                    Position { x, y },
                    func(self.get_part(
                        Space {
                            position: Position {
                                x: shift(x),
                                y: shift(y),
                            },
                            size: Dimension {
                                width: r,
                                height: r,
                            },
                        })),
                );
            }
        }
    }
}

