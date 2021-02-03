use args::ImageArgs;
use image::open;
use ndarray::{ArrayView, Axis, ShapeBuilder};

mod args;

fn main() {
    let images_infoes= ImageArgs::parse_args().unwrap();

    // Image is in format rgba
    let image = open("/home/geo/src/ImageProcessing/floor.bmp").expect("cannot open image");

    println!("{}", image.as_bytes().len() / 4)
}
