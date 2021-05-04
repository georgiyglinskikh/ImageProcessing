use crate::images::utils::{PixelType, ImageType};
use crate::images::part::Part;
use crate::images::types::{Space, Position, Dimension};

pub trait Filter: Part {
    fn filter(&mut self, r: u32, func: fn(ImageType) -> PixelType);
}

impl Filter for ImageType {
    fn filter(&mut self, r: u32, func: fn(ImageType) -> PixelType) {
        let half_r = r / 2;

        let shift = |x: u32| x as isize - half_r as isize;

        for x in 0..self.width() {
            for y in 0..self.height() {
                self.put_pixel(
                    x,
                    y,
                    func(self.get_part(
                        Space {
                            position: Position {
                                x: shift(x),
                                y: shift(y),
                            },
                            size: Dimension {
                                width: r as isize,
                                height: r as isize
                            }
                        }))
                );
            }
        }
    }
}

