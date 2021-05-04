use crate::images::utils::ImageType;
use image::GenericImage;

pub trait Part {
    fn get_part(&self, x: i32, y: i32, width: i32, height: i32) -> Self;
}

impl Part for ImageType {
    fn get_part(&self, x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut result = self.clone();

        // TODO: hide this logic
        let width =
            if x + width >= result.width() as i32 {
                result.width() as i32 - x
            } else if x < 0 {
                width + x
            } else {
                width
            };

        let height =
            if y + height >= result.height() as i32 {
                result.height() as i32 - y
            } else if y < 0 {
                height + y
            } else {
                height
            };

        let x =
            if x < 0 {
                0
            } else {
                x
            };

        let y =
            if y < 0 {
                0
            } else {
                y
            };

        let result = result.sub_image(x as u32, y as u32, width as u32, height as u32);

        result.to_image()
    }
}