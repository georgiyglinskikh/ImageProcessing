#![feature(array_zip)]
#![feature(array_map)]

mod images;

use image::Rgba;
use crate::images::utils::white_black::WhiteBlackTypes;
use crate::images::white_black::WhiteBlack;
use std::borrow::Borrow;

fn main() {
    let mut image =
        image::open("/home/geo/PycharmProjects/ImageProcessing/images/floor.bmp").expect("Cannot open image");

    let mut buf = image.as_mut_rgba8().expect("Cannot open image in this format");

    WhiteBlack::white_black(buf, WhiteBlackTypes::Smooth1);

    image.save("./a.bmp").expect("Cannot save an image")
}
